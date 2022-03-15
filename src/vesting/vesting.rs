use std::collections::HashMap;
use near_sdk::AccountId;
use crate::types::ScheduleId;
use crate::vesting::beneficiary::Beneficiary;
use crate::vesting::interfaces::Owner;
use crate::vesting::schedule::models::Schedule;
use crate::vesting::schedule::Schedule;

struct Vesting {
    owner: AccountId,
    beneficiary: AccountId,
    schedules: HashMap<ScheduleId, Schedule>,
    title: String,
    description: String
}