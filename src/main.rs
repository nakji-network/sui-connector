mod sui_system;

use std::sync::mpsc;
use std::time::{Duration, SystemTime};

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
use sui_sdk::{SuiClient, SuiClientBuilder};

#[derive(Parser, Debug)]
struct Args {
    /// Backfill range in seconds (<0: Don't Backfill, =0: Backfill all)
    #[arg(short, long, default_value_t = 0)]
    duration: i64,
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

    let (tx, rx) = mpsc::channel::<Event>();

    if args.duration >= 0 {
        let sui2 = sui.clone();
        let tx2 = tx.clone();

        tokio::spawn(async move {
            println!("backfill started");
            match backfill(sui2, tx2, args.duration as u64).await {
                Err(err) => {
                    println!("backfill failed: {}", err)
                }
                Ok(_) => {
                    println!("backfill stopped");
                }
            }
        });
    }

    tokio::spawn(async move {
        if let Err(err) = subscribe(sui, tx).await {
            println!("subscribe failed: {}", err)
        }
    });

    for event in rx {
        if let Err(err) = handle_event(&mut c, event).await {
            println!("{}", err)
        }
    }

    Ok(())
}

async fn backfill(sui: SuiClient, tx: mpsc::Sender<Event>, duration: u64) -> anyhow::Result<()> {
    let filter = event_filter(duration)?;

    let mut ss = sui
        .event_api()
        .get_events_stream(filter, None, true)
        .boxed();

    while let Some(event) = ss.next().await {
        tx.send(Event {
            t: MessageType::BF,
            e: event,
        })?
    }

    Ok(())
}

async fn subscribe(sui: SuiClient, tx: mpsc::Sender<Event>) -> anyhow::Result<()> {
    let filter = event_filter(0)?;

    let mut ss = sui.event_api().subscribe_event(filter).await?;

    while let Some(event) = ss.next().await {
        tx.send(Event {
            t: MessageType::FCT,
            e: event?,
        })?
    }

    Ok(())
}

async fn handle_event(c: &mut Connector, event: Event) -> anyhow::Result<()> {
    let e = event.e;
    let t = event.t;

    match e.type_.name.to_string().as_str() {
        "ValidatorEpochInfoEventV2" => match e.timestamp_ms {
            Some(timestamp_ms) => {
                let data = serde_json::from_value::<ValidatorEpochInfoEventV2JSON>(e.parsed_json)?;
                let proto_msg = data.protobuf_message(timestamp_ms, &e.id)?;

                let topic = topic(&c, Box::new(ValidatorEpochInfoEventV2::new()), t.clone());
                let key = Key::new(String::from(""), String::from(""));
                let msg = Message::new(topic, key, proto_msg);

                c.producer.produce_transactional_messages(vec![msg]).await?;

                println!("{:?} -> {} {}", t, e.transaction_module, e.type_);

                return Ok(());
            }
            None => {
                bail!("event without timestamp: {:?}", e.id)
            }
        },
        _ => {
            bail!("unhandled event: {} {}", e.transaction_module, e.type_)
        }
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

fn event_filter(duration: u64) -> anyhow::Result<EventFilter> {
    let tag =
        sui_sdk::types::parse_sui_struct_tag("0x3::validator_set::ValidatorEpochInfoEventV2")?;

    let mut filter = EventFilter::MoveEventType(tag);

    if duration > 0 {
        let start_time = (SystemTime::now() - Duration::from_secs(duration))
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_millis() as u64;

        let end_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_millis() as u64;

        let time_range = EventFilter::TimeRange {
            start_time,
            end_time,
        };

        filter = filter.and(time_range);
    }

    return Ok(filter);
}
