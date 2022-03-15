pub mod natural_time_linear_release;

use crate::AccountId;
use crate::types::{Amount, SecondTimeStamp};

pub enum Schedule {
    NaturalTimeLinearRelease
}

pub trait NaturalTime {
    fn get_start_time(&self)->SecondTimeStamp;
    fn get_end_time(&self)->SecondTimeStamp;
    fn get_period(&self)->SecondTimeStamp {
        self.get_end_time()-self.get_start_time()+1
    }
}

pub trait VestingAmount {
    fn get_released_amount(&self)->Amount {
        assert!(self.get_total_amount()>=self.get_unreleased_amount(),"total amount should ge released amount.");
        self.get_total_amount()-self.get_unreleased_amount()
    }
    fn get_unreleased_amount(&self)->Amount;
    fn get_total_amount(&self)->Amount;
    fn get_claimable_amount(&self)->Amount {
        self.get_balance()-self.get_unreleased_amount()
    }
}

pub trait IBalance {
    fn get_token_id(&self)-> &AccountId;
    fn get_balance(&self)-> Amount;
    fn set_balance(&mut self, balance: Amount);
}

impl<T:NaturalTime+IBalance> VestingAmount for T {
    fn get_unreleased_amount(&self) -> Amount {

    }
    fn get_total_amount(&self) -> Amount {
        return self.
    }
}

pub trait Claimable: VestingAmount {

}