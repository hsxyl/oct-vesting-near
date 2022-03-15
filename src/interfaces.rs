use crate::types::{ScheduleId, SecondTimeStamp};
use crate::VestingId;

pub trait VestingOwnerAction {

}

pub trait VestingBeneficiaryAction {
   fn claim(&mut self, vesting_id: VestingId, schedule: Option<Vec<Se>>);


}

pub trait VestingView {
   fn view_claim_at(&self, time: SecondTimeStamp, schedule: ScheduleId);
}