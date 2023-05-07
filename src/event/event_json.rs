use std::time::{Duration, SystemTime};

use crate::event::event::SwappedEvent;

use protobuf::well_known_types::timestamp::Timestamp;
use serde::Deserialize;
use sui_sdk::types::event::EventID;

#[derive(Deserialize, Debug)]
pub struct SwappedEventJSON {
    global: String,
    lp_name: String,
    coin_x_in: String,
    coin_x_out: String,
    coin_y_in: String,
    coin_y_out: String,
}

impl SwappedEventJSON {
    pub fn protobuf_message(
        self,
        timestamp_ms: u64,
        event_id: &EventID,
    ) -> anyhow::Result<SwappedEvent> {
        let mut data = SwappedEvent::new();

        data.ts = protobuf::MessageField::some(Timestamp::from(
            SystemTime::UNIX_EPOCH + Duration::from_millis(timestamp_ms),
        ));
        data.tx_digest = event_id.tx_digest.inner().to_vec();
        data.event_seq = event_id.event_seq;
        data.global = hex::decode(self.global.trim_start_matches("0x"))?;
        data.lp_name = self.lp_name.parse()?;
        data.coin_x_in = self.coin_x_in.parse()?;
        data.coin_x_out = self.coin_x_out.parse()?;
        data.coin_y_in = self.coin_y_in.parse()?;
        data.coin_y_out = self.coin_y_out.parse()?;

        Ok(data)
    }
}
