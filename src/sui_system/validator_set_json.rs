use std::time::{Duration, SystemTime};

use crate::sui_system::validator_set::{PoolTokenExchangeRate, ValidatorEpochInfoEventV2};

use anyhow::Ok;
use protobuf::well_known_types::timestamp::Timestamp;
use serde::Deserialize;
use sui_sdk::types::event::EventID;

#[derive(Deserialize, Debug)]
pub struct PoolTokenExchangeRateJSON {
    pub pool_token_amount: String,
    pub sui_amount: String,
}

impl PoolTokenExchangeRateJSON {
    pub fn protobuf_message(self) -> anyhow::Result<PoolTokenExchangeRate> {
        let mut data = PoolTokenExchangeRate::new();
        data.pool_token_amount = self.pool_token_amount.parse()?;
        data.sui_amount = self.sui_amount.parse()?;
        return Ok(data);
    }
}

#[derive(Deserialize, Debug)]
pub struct ValidatorEpochInfoEventV2JSON {
    pub epoch: String,
    pub validator_address: String,
    pub reference_gas_survey_quote: String,
    pub stake: String,
    pub commission_rate: String,
    pub pool_staking_reward: String,
    pub storage_fund_staking_reward: String,
    pub pool_token_exchange_rate: PoolTokenExchangeRateJSON,
    pub tallying_rule_reporters: Vec<String>,
    pub tallying_rule_global_score: String,
}

impl ValidatorEpochInfoEventV2JSON {
    pub fn protobuf_message(
        self,
        timestamp_ms: u64,
        event_id: &EventID,
    ) -> anyhow::Result<ValidatorEpochInfoEventV2> {
        let mut data = ValidatorEpochInfoEventV2::new();
        data.ts = protobuf::MessageField::some(Timestamp::from(
            SystemTime::UNIX_EPOCH + Duration::from_millis(timestamp_ms),
        ));
        data.tx_digest = event_id.tx_digest.inner().to_vec();
        data.event_seq = event_id.event_seq;
        data.epoch = self.epoch.parse()?;
        data.validator_address = hex::decode(self.validator_address.trim_start_matches("0x"))?;
        data.reference_gas_survey_quote = self.reference_gas_survey_quote.parse()?;
        data.stake = self.stake.parse()?;
        data.commission_rate = self.commission_rate.parse()?;
        data.pool_staking_reward = self.pool_staking_reward.parse()?;
        data.storage_fund_staking_reward = self.storage_fund_staking_reward.parse()?;
        data.pool_token_exchange_rate =
            protobuf::MessageField::some(self.pool_token_exchange_rate.protobuf_message()?);
        for tallying_rule_reporter in self.tallying_rule_reporters {
            data.tallying_rule_reporters
                .push(hex::decode(tallying_rule_reporter)?);
        }
        data.tallying_rule_global_score = self.tallying_rule_global_score.parse()?;
        return Ok(data);
    }
}
