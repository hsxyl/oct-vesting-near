use near_sdk::json_types::U128;
use crate::AccountId;
use crate::types::{Amount, SecondTimeStamp};
use crate::vesting::schedule::{IBalance, NaturalTime, VestingAmount};
use crate::vesting::schedule::traits::{NaturalTime, VestingAmount};
use crate::vesting::*;
use crate::vesting::schedule::Schedule::NaturalTimeLinearRelease;

pub struct NaturalTimeLinearRelease {
    start_time: SecondTimeStamp,
    end_time: SecondTimeStamp,
    token_id: AccountId,
    token_balance: Amount,
    total_amount: Amount,
}

impl NaturalTime for NaturalTimeLinearRelease {
    fn get_start_time(&self) -> SecondTimeStamp {
        self.start_time
    }

    fn get_end_time(&self) -> SecondTimeStamp {
        self.end_time
    }
}

impl IBalance for NaturalTimeLinearRelease {
    fn get_token_id(&self) -> &AccountId {
        &self.token_id
    }

    fn get_balance(&self) -> Amount {
        self.token_balance
    }

    fn set_balance(&mut self, balance: Amount) {
        self.token_balance = balance
    }
}

impl VestingAmount for NaturalTimeLinearRelease {
    fn get_unreleased_amount(&self) -> Amount {
        u64::try_from(self.get_total_amount()) self.get_total_amount()
        todo!()
    }

    fn get_total_amount(&self) -> Amount {
        self.total_amount
    }
}