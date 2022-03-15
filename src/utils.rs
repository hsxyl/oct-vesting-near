use near_sdk::env;
use crate::types::SecondTimeStamp;

pub fn get_block_second_time() -> SecondTimeStamp {
    return env::block_timestamp()/1000_000_000;
}