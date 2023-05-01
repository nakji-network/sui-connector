mod sui_system;

use std::sync::mpsc;
use std::vec;

use crate::sui_system::validator_set::ValidatorEpochInfoEventV2;
use crate::sui_system::validator_set_json::ValidatorEpochInfoEventV2JSON;

use anyhow::bail;
use clap::Parser;
use futures::StreamExt;
use nakji_connector::connector::Connector;
use nakji_connector::kafka_utils::key::Key;
use nakji_connector::kafka_utils::{topic, Message, MessageType, Topic};
use protobuf::MessageDyn;
use serde_json;
use sui_sdk::rpc_types::{EventFilter, SuiEvent};
use sui_sdk::types::event::EventID;
use sui_sdk::{SuiClient, SuiClientBuilder};

const CHANNEL_SIZE: usize = 1000;

const QUERY_PAGE_SZIE: usize = 100;

const EVENT_TYPES: &'static [&'static str] = &["0x3::validator_set::ValidatorEpochInfoEventV2"];

#[derive(Parser, Debug)]
struct Args {
    /// Limit number of events to backfill (<0: Don't Backfill, =0: Backfill all)
    #[arg(short, long, default_value_t = 0)]
    limit: i64,
}

struct Event {
    t: MessageType,
    e: SuiEvent,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut c = Connector::new();

    c.register_protos(
        MessageType::FCT,
        vec![Box::new(ValidatorEpochInfoEventV2::new())],
    )
    .await;

    c.register_protos(
        MessageType::BF,
        vec![Box::new(ValidatorEpochInfoEventV2::new())],
    )
    .await;

    let ws_url = c.config.sub_config["ws_url"]
        .as_str()
        .unwrap_or("ws://127.0.0.1:9000");

    let http_url = c.config.sub_config["http_url"]
        .as_str()
        .unwrap_or("http://127.0.0.1:9000");

    println!("connect to [{}] [{}]", ws_url, http_url);

    let sui = SuiClientBuilder::default()
        .ws_url(ws_url)
        .build(http_url)
        .await?;

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
                println!("query [{}] started", event_type);
                match query_events(sui, tx, filter, limit).await {
                    Ok(_) => println!("query [{}] stopped", event_type),
                    Err(err) => println!("query [{}] failed: {}", event_type, err),
                }
            });
        }
    }

    tokio::spawn(async move {
        println!("subscription started");
        match subscribe(sui, tx).await {
            Ok(_) => println!("subscription stopped"),
            Err(err) => println!("subscription failed: {}", err),
        }
    });

    for event in rx {
        if let Err(err) = handle_event(&mut c, event).await {
            println!("{}", err)
        }
    }

    Ok(())
}

async fn subscribe(sui: SuiClient, tx: mpsc::SyncSender<Event>) -> anyhow::Result<()> {
    let mut event_types = Vec::new();
    for event_type in EVENT_TYPES {
        event_types.push(EventFilter::MoveEventType(
            sui_sdk::types::parse_sui_struct_tag(event_type)?,
        ))
    }
    let filter = EventFilter::Any(event_types);

    let mut ss = sui.event_api().subscribe_event(filter).await?;

    while let Some(event) = ss.next().await {
        tx.send(Event {
            t: MessageType::FCT,
            e: event?,
        })?
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

                for event in page.data {
                    tx.send(Event {
                        t: MessageType::BF,
                        e: event,
                    })?
                }

                limit -= query_size;
                is_first = false;
                cursor = page.next_cursor;
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
            "0x3::validator_set::ValidatorEpochInfoEventV2" => {
                let data = serde_json::from_value::<ValidatorEpochInfoEventV2JSON>(e.parsed_json)?;
                let proto_msg = data.protobuf_message(timestamp_ms, &e.id)?;

                let topic = topic(&c, Box::new(ValidatorEpochInfoEventV2::new()), t.clone());
                let key = Key::new(String::from(""), String::from(""));
                let msg = Message::new(topic, key, proto_msg);

                c.producer.produce_transactional_messages(vec![msg]).await?;

                println!("{:?} -> {}", t, e.type_);

                return Ok(());
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
