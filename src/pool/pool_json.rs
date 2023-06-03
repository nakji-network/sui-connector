use std::time::{Duration, SystemTime};

use crate::pool::pool::SwappedEvent;

use protobuf::well_known_types::timestamp::Timestamp;
use serde::Deserialize;
use sui_sdk::types::event::EventID;

#[derive(Deserialize, Debug)]
pub struct SwappedEventJSON {
    atob: bool,
    pool: String,
    partner: String,
    amount_in: String,
    amount_out: String,
    ref_amount: String,
    fee_amount: String,
    vault_a_amount: String,
    vault_b_amount: String,
    before_sqrt_price: String,
    after_sqrt_price: String,
    steps: String,
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
        data.atob = self.atob;
        data.pool = hex::decode(self.pool.trim_start_matches("0x"))?;
        data.partner = hex::decode(self.partner.trim_start_matches("0x"))?;
        data.amount_in = self.amount_in.parse()?;
        data.amount_out = self.amount_out.parse()?;
        data.ref_amount = self.ref_amount.parse()?;
        data.fee_amount = self.fee_amount.parse()?;
        data.vault_a_amount = self.vault_a_amount.parse()?;
        data.vault_b_amount = self.vault_b_amount.parse()?;
        data.before_sqrt_price = self
            .before_sqrt_price
            .parse::<u128>()?
            .to_be_bytes()
            .to_vec();
        data.after_sqrt_price = self
            .after_sqrt_price
            .parse::<u128>()?
            .to_be_bytes()
            .to_vec();
        data.steps = self.steps.parse()?;

        Ok(data)
    }
}
