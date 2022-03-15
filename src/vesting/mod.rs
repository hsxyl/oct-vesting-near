pub mod interfaces;
pub mod eth_interface;
pub mod schedule;
pub mod beneficiary;
pub mod vesting;

enum VestingType {
    UnsupervisedTimeLock,
    SupervisedTimeLock,
    SupervisedMultiTimeLock
}

struct UnsupervisedTimeLockVesting {


}
