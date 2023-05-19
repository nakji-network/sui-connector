mod event;

use std::collections::HashMap;
use std::sync::mpsc;
use std::time::Duration;
use std::vec;

use anyhow::bail;
use clap::Parser;
use event::event::SwappedEvent;
use event::event_json::SwappedEventJSON;
use futures::StreamExt;
use log::{debug, error, info, warn};
use nakji_connector::connector::Connector;
use nakji_connector::kafka_utils::key::Key;
use nakji_connector::kafka_utils::{topic, Message, MessageType, Topic};
use protobuf::MessageDyn;
use serde_json;
use sui_sdk::rpc_types::{EventFilter, SuiEvent};
use sui_sdk::types::event::EventID;
use sui_sdk::{SuiClient, SuiClientBuilder};
use tokio::time::timeout;

const CHANNEL_SIZE: usize = 1000;

const QUERY_PAGE_SZIE: usize = 1000;

const WEBSOCKET_TIMEOUT: Duration = Duration::from_secs(30 * 60);

const EVENT_TYPES: &'static [&'static str] =
    &["0x6b84da4f5dc051759382e60352377fea9d59bc6ec92dc60e0b6387e05274415f::event::SwappedEvent"];

#[derive(Parser, Debug)]
struct Args {
    /// Limit number of events to backfill (<0: Don't backfill, =0: Backfill all)
    #[arg(short = 'b', long, default_value_t = 0)]
    limit: i64,

    /// Log level (e.g. off, trace, debug, info, warn, error)
    #[arg(short, long, default_value_t = String::from("debug"))]
    log_level: String,
}

struct Event {
    t: MessageType,
    e: SuiEvent,
}

#[derive(Clone)]
struct SuiClientWrapper {
    ws_url: String,
    http_url: String,
    client: SuiClient,
}

impl SuiClientWrapper {
    async fn new(ws_url: String, http_url: String) -> anyhow::Result<Self> {
        let client = Self::build_client(ws_url.clone(), http_url.clone()).await?;
        Ok(Self {
            ws_url,
            http_url,
            client,
        })
    }

    async fn build_client(ws_url: String, http_url: String) -> anyhow::Result<SuiClient> {
        let client = SuiClientBuilder::default()
            .ws_url(ws_url)
            .build(http_url)
            .await?;
        Ok(client)
    }

    async fn reconnect(&mut self) -> anyhow::Result<()> {
        self.client = Self::build_client(self.ws_url.clone(), self.http_url.clone()).await?;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let log_level = match args.log_level.as_str() {
        "off" => stderrlog::LogLevelNum::Off,
        "trace" => stderrlog::LogLevelNum::Trace,
        "debug" => stderrlog::LogLevelNum::Debug,
        "info" => stderrlog::LogLevelNum::Info,
        "warn" => stderrlog::LogLevelNum::Warn,
        "error" => stderrlog::LogLevelNum::Error,
        _ => bail!("invalid log_level: {}", args.log_level),
    };

    stderrlog::new()
        .timestamp(stderrlog::Timestamp::Second)
        .verbosity(log_level)
        .init()?;

    let mut c = Connector::new();

    info!("connect to kafka [{}]", c.config.kafka_url);
    info!(
        "connect to proto registry [{}]",
        c.config.proto_registry_host
    );

    c.register_protos(MessageType::FCT, vec![Box::new(SwappedEvent::new())])
        .await;

    c.register_protos(MessageType::BF, vec![Box::new(SwappedEvent::new())])
        .await;

    let ws_url = c.config.sub_config["ws_url"]
        .as_str()
        .unwrap_or("ws://127.0.0.1:9000");

    let http_url = c.config.sub_config["http_url"]
        .as_str()
        .unwrap_or("http://127.0.0.1:9000");

    info!("connect to SUI [{}] [{}]", ws_url, http_url);

    let sui = SuiClientWrapper::new(ws_url.to_string(), http_url.to_string()).await?;

    let (tx, rx) = mpsc::sync_channel::<Event>(CHANNEL_SIZE);

    if args.limit >= 0 {
        for event_type in EVENT_TYPES {
            let sui = sui.clone();
            let tx = tx.clone();
            let filter =
                EventFilter::MoveEventType(sui_sdk::types::parse_sui_struct_tag(event_type)?);
            let limit = if args.limit == 0 {
                None
            } else {
                Some(args.limit as usize)
            };

            tokio::spawn(async move {
                info!("query [{}] started", event_type);
                match query_events(sui.client.clone(), tx, filter, limit).await {
                    Ok(_) => info!("query [{}] stopped", event_type),
                    Err(err) => error!("query [{}] failed: {}", event_type, err),
                }
            });
        }
    }

    tokio::spawn(async move {
        info!("subscription started");
        match subscribe(sui, tx).await {
            Ok(_) => info!("subscription stopped"),
            Err(err) => error!("subscription failed: {}", err),
        }
    });

    for event in rx {
        if let Err(err) = handle_event(&mut c, event).await {
            error!("{}", err)
        }
    }

    Ok(())
}

async fn subscribe(mut sui: SuiClientWrapper, tx: mpsc::SyncSender<Event>) -> anyhow::Result<()> {
    let mut event_types = Vec::new();
    for event_type in EVENT_TYPES {
        event_types.push(EventFilter::MoveEventType(
            sui_sdk::types::parse_sui_struct_tag(event_type)?,
        ))
    }
    let filter = EventFilter::Any(event_types);

    let mut ss = sui
        .client
        .event_api()
        .subscribe_event(filter.clone())
        .await?;

    // If there is possible gap for a certain event type
    let mut possible_gaps = HashMap::<String, bool>::new();

    // Store id of the last event recevied for each event type
    let mut last_event_ids = HashMap::<String, EventID>::new();

    loop {
        match timeout(WEBSOCKET_TIMEOUT, ss.next()).await {
            Ok(iter) => match iter {
                Some(result) => {
                    let event = result?;
                    let event_type = event.type_.to_string();
                    let possible_gap = possible_gaps.get(&event_type).unwrap_or(&false).clone();
                    let last_event_id = last_event_ids.get(&event_type);

                    if possible_gap && last_event_id.is_some() {
                        let from = last_event_id.unwrap().clone();
                        let to = event.id.clone();
                        info!(
                            "start querying gap for [{}] from [{:?}] to [{:?}]",
                            event_type, from, to
                        );
                        let filter = EventFilter::MoveEventType(
                            sui_sdk::types::parse_sui_struct_tag(event_type.as_str())?,
                        );
                        query_gap(
                            sui.client.clone(),
                            tx.clone(),
                            filter,
                            from.clone(),
                            to.clone(),
                        )
                        .await?;
                        info!(
                            "stop querying gap for [{}] from [{:?}] to [{:?}]",
                            event_type, from, to
                        );
                    }

                    possible_gaps.insert(event_type.clone(), false);
                    last_event_ids.insert(event_type, event.id.clone());

                    tx.send(Event {
                        t: MessageType::FCT,
                        e: event,
                    })?
                }
                None => break,
            },
            Err(_) => {
                warn!(
                    "websocket inactive for {}s, initiate new connection",
                    WEBSOCKET_TIMEOUT.as_secs()
                );

                for v in possible_gaps.values_mut() {
                    *v = true;
                }

                sui.reconnect().await?;

                ss = sui
                    .client
                    .event_api()
                    .subscribe_event(filter.clone())
                    .await?
            }
        }
    }

    while let Some(event) = ss.next().await {
        tx.send(Event {
            t: MessageType::FCT,
            e: event?,
        })?
    }

    Ok(())
}

async fn query_gap(
    sui: SuiClient,
    tx: mpsc::SyncSender<Event>,
    filter: EventFilter,
    from: EventID,
    to: EventID,
) -> anyhow::Result<()> {
    let mut cursor = Some(from.clone());

    'outer: while cursor.is_some() {
        let page = sui
            .event_api()
            .query_events(filter.clone(), cursor, Some(QUERY_PAGE_SZIE), false)
            .await?;
        for event in page.data {
            if event.id == from {
                continue;
            }
            if event.id == to {
                break 'outer;
            }
            tx.send(Event {
                t: MessageType::FCT,
                e: event,
            })?
        }
        cursor = page.next_cursor;
    }

    Ok(())
}

async fn query_events(
    sui: SuiClient,
    tx: mpsc::SyncSender<Event>,
    query: EventFilter,
    limit: Option<usize>,
) -> anyhow::Result<()> {
    match limit {
        None => {
            let mut ss = sui.event_api().get_events_stream(query, None, true).boxed();

            while let Some(event) = ss.next().await {
                tx.send(Event {
                    t: MessageType::BF,
                    e: event,
                })?
            }
        }
        Some(mut limit) => {
            let mut is_first = true;
            let mut cursor: Option<EventID> = None;

            while limit > 0 && (is_first || cursor.is_some()) {
                let query_size = if limit > QUERY_PAGE_SZIE {
                    QUERY_PAGE_SZIE
                } else {
                    limit
                };

                let page = sui
                    .event_api()
                    .query_events(query.clone(), cursor, Some(query_size), true)
                    .await?;

                is_first = false;
                cursor = page.next_cursor;
                limit -= page.data.len();

                for event in page.data {
                    tx.send(Event {
                        t: MessageType::BF,
                        e: event,
                    })?
                }
            }
        }
    }

    Ok(())
}

async fn handle_event(c: &mut Connector, event: Event) -> anyhow::Result<()> {
    let e = event.e;
    let t = event.t;

    match e.timestamp_ms {
        None => bail!("event without timestamp: {:?}", e.id),
        Some(timestamp_ms) => match e.type_.to_string().as_str() {
            "0x6b84da4f5dc051759382e60352377fea9d59bc6ec92dc60e0b6387e05274415f::event::SwappedEvent" => {
                let data = serde_json::from_value::<SwappedEventJSON>(e.parsed_json)?;
                let proto_msg = data.protobuf_message(timestamp_ms, &e.id)?;

                let topic = topic(&c, Box::new(SwappedEvent::new()), t.clone());
                let key = Key::new(String::from(""), String::from(""));
                let msg = Message::new(topic, key, proto_msg);

                c.producer.produce_transactional_messages(vec![msg]).await?;

                debug!("{:?} -> {}", t, e.type_);

                Ok(())
            }
            _ => bail!("unhandled event: {}", e.type_),
        },
    }
}

fn topic(c: &Connector, msg: Box<dyn MessageDyn>, t: MessageType) -> Topic {
    let event_name = topic::get_event_name(msg.clone_box());

    let topic = Topic::new(
        c.config.kafka_env.clone(),
        t,
        c.manifest.author.clone(),
        c.manifest.name.clone(),
        c.manifest.version.clone(),
        event_name,
    );

    return topic;
}
