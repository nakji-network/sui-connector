mod sui_system;

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
use sui_sdk::{SuiClient, SuiClientBuilder};

#[derive(Parser, Debug)]
pub struct Args {
    /// Full node address
    #[arg(short, long, default_value_t = String::from("127.0.0.1:9000"))]
    addr: String,

    /// Whether to use secure connection
    #[arg(short, long, default_value_t = false)]
    secured: bool,
}

impl Args {
    pub fn ws_url(&self) -> String {
        return if self.secured {
            format!("wss://{}", self.addr)
        } else {
            format!("ws://{}", self.addr)
        };
    }

    pub fn http_url(&self) -> String {
        return if self.secured {
            format!("https://{}", self.addr)
        } else {
            format!("http://{}", self.addr)
        };
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let sui = SuiClientBuilder::default()
        .ws_url(args.ws_url())
        .build(args.http_url())
        .await?;

    let sui2 = sui.clone();

    tokio::spawn(async move {
        if let Err(err) = backfill(sui2).await {
            println!("{}", err)
        }
    });

    subscribe(sui).await?;

    Ok(())
}

async fn backfill(sui: SuiClient) -> anyhow::Result<()> {
    println!("start backfill");

    let mut c = Connector::new();

    c.register_protos(
        MessageType::BF,
        vec![Box::new(ValidatorEpochInfoEventV2::new())],
    )
    .await;

    let package = ObjectID::from_hex_literal(
        "0x0000000000000000000000000000000000000000000000000000000000000003",
    )?;
    let filter = EventFilter::All(vec![EventFilter::Package(package)]);

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

    let package = ObjectID::from_hex_literal(
        "0x0000000000000000000000000000000000000000000000000000000000000003",
    )?;
    let filter = EventFilter::All(vec![EventFilter::Package(package)]);

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

                println!("{:?} -> {}", t, event.type_);

                return Ok(());
            }
            None => {
                bail!("event without timestamp: {:?}", event.id)
            }
        },
        _ => {
            bail!("unhandled event: {}", event.type_)
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
