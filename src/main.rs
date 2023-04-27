mod sui_system;

use std::time::{Duration, SystemTime};

use crate::sui_system::validator_set::ValidatorEpochInfoEventV2;
use crate::sui_system::validator_set_json::ValidatorEpochInfoEventV2JSON;

use anyhow::{bail, Ok};
use clap::Parser;
use futures::StreamExt;
use nakji_connector::connector::Connector;
use nakji_connector::kafka_utils::key::Key;
use nakji_connector::kafka_utils::{topic, Message, MessageType, Topic};
use protobuf::MessageDyn;
use serde_json;
use sui_sdk::rpc_types::{EventFilter, SuiEvent};
use sui_sdk::types::base_types::ObjectID;
use sui_sdk::types::Identifier;
use sui_sdk::{SuiClient, SuiClientBuilder};

#[derive(Parser, Debug)]
pub struct Args {
    /// Websocket RPC endpoint
    #[arg(long, default_value_t = String::from("ws://127.0.0.1:9000"))]
    ws_url: String,

    /// HTTP RPC endpoint
    #[arg(long, default_value_t = String::from("http://127.0.0.1:9000"))]
    http_url: String,

    /// Number of seconds to backfill (0 means backfill all)
    #[arg(short, long, default_value_t = 0)]
    duration: u64,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let sui = SuiClientBuilder::default()
        .ws_url(args.ws_url)
        .build(args.http_url)
        .await?;

    let sui2 = sui.clone();

    tokio::spawn(async move {
        if let Err(err) = backfill(sui2, args.duration).await {
            println!("{}", err)
        }
    });

    subscribe(sui).await?;

    Ok(())
}

async fn backfill(sui: SuiClient, duration: u64) -> anyhow::Result<()> {
    println!("start backfill");

    let mut c = Connector::new();

    c.register_protos(
        MessageType::BF,
        vec![Box::new(ValidatorEpochInfoEventV2::new())],
    )
    .await;

    let filter = event_filter(duration)?;

    let mut ss = sui
        .event_api()
        .get_events_stream(filter, None, true)
        .boxed();
    while let Some(event) = ss.next().await {
        if let Err(err) = handle_event(&mut c, event, MessageType::BF).await {
            println!("{}", err)
        }
    }

    println!("stop backfill");

    Ok(())
}

async fn subscribe(sui: SuiClient) -> anyhow::Result<()> {
    let mut c = Connector::new();

    c.register_protos(
        MessageType::FCT,
        vec![Box::new(ValidatorEpochInfoEventV2::new())],
    )
    .await;

    let filter = event_filter(0)?;

    let mut ss = sui.event_api().subscribe_event(filter).await?;
    while let Some(event) = ss.next().await {
        if let Err(err) = handle_event(&mut c, event?, MessageType::FCT).await {
            println!("{}", err)
        }
    }

    Ok(())
}

async fn handle_event(c: &mut Connector, event: SuiEvent, t: MessageType) -> anyhow::Result<()> {
    match event.type_.name.to_string().as_str() {
        "ValidatorEpochInfoEventV2" => match event.timestamp_ms {
            Some(timestamp_ms) => {
                let data =
                    serde_json::from_value::<ValidatorEpochInfoEventV2JSON>(event.parsed_json)?;
                let proto_msg = data.protobuf_message(timestamp_ms, &event.id)?;

                let topic = topic(&c, Box::new(ValidatorEpochInfoEventV2::new()), t.clone());
                let key = Key::new(String::from(""), String::from(""));
                let msg = Message::new(topic, key, proto_msg);

                c.producer.produce_transactional_messages(vec![msg]).await?;

                println!("{:?} -> {} {}", t, event.transaction_module, event.type_);

                return Ok(());
            }
            None => {
                bail!("event without timestamp: {:?}", event.id)
            }
        },
        _ => {
            bail!(
                "unhandled event: {} {}",
                event.transaction_module,
                event.type_
            )
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
    let package = ObjectID::from_hex_literal(
        "0x0000000000000000000000000000000000000000000000000000000000000003",
    )?;

    let module = Identifier::new(String::from("sui_system"))?;

    let mut filter = EventFilter::All(vec![EventFilter::MoveModule { package, module }]);

    if duration > 0 {
        let start_time = (SystemTime::now() + Duration::from_secs(duration))
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
