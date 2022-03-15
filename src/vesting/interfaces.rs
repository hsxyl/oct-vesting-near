use near_sdk::json_types::U128;
use crate::types::{Amount, SecondTimeStamp};
use crate::vesting::schedule::VestingAmount;

pub trait Owner {
    fn get_owner(self);
}

pub trait OwnerAction {

}

pub trait BeneficiaryAction {

}

pub trait Claimable<T> where T: VestingAmount {
    fn claim(t:&mut T)->Amount {
        let claimable_amount = t.get_claimable_amount();
        assert!(t.get_balance()>=claimable_amount);
        t.set_balance(t.get_balance()-claimable_amount);
        claimable_amount
    }
}