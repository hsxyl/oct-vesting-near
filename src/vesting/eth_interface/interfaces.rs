// impl old vesting contract in eth: https://github.com/octopus-network/oct-token-eth/tree/main/contracts

use near_sdk::json_types::U128;
use crate::AccountId;
use crate::types::{Amount, Day, SecondTimeStamp};

// old vesting contract in eth: https://github.com/octopus-network/oct-token-eth/blob/main/contracts/UnsupervisedTimelock.sol
pub trait UnsupervisedTimeLock
{
    /**
     * @return the beneficiary address
     */
    fn beneficiary() -> AccountId;

    /**
     * @return the total balance of benefit
     */
    fn total_benefit()-> Amount;

    /**
     * @return the balance to release for the beneficiary at the moment
     */
    fn released_balance() -> Amount;

    /**
     * @return the unreleased balance of the beneficiary at the moment
     */
    fn unreleased_balance() -> Amount;

    /**
     * @return the withdrawed balance of beneficiary
     */
    fn withdraw_balance() -> Amount;

    /**
     * @notice Withdraws tokens to beneficiary
     */
    fn withdraw();
}

trait SupervisedTimeLock: UnsupervisedTimeLock {
    /**
     * @notice Teminate this contract and withdraw all amount of unreleased balance to the owner.
     * After the contract is terminated, the beneficiary can still withdraw all amount of
     * released balance.
     */
    fn terminate();
}

pub trait SupervisedMultiTimeLock {

    /**
     * @return the issued benefit of a beneficiary
     */
    fn issued_benefit_of(account: AccountId)->Amount;

    /**
     * @return the amount which can be withdrawn by a beneficiary at the moment
     */
    fn released_amount_of(account: AccountId)->Amount;

    /**
     * @return the unreleased amount of a beneficiary at the moment
     */
    fn unreleased_amount_of(account: AccountId)->Amount;

    /**
     * @return the withdrawn amount of a beneficiary at the moment
     */
    fn withdrawn_amount_of(account: AccountId)->Amount;

    /**
     * @notice Issue a certain amount benefit to a beneficiary.
     * An address of a beneficiary can only be used once.
     */
    fn issue_benefit_to(
        account: AccountId,
        total_amount: Amount,
        release_start_time: SecondTimeStamp,
        days_of_time_lock: Day
    );

    /**
     * @notice Withdraws benefit of a beneficiary.
     * This function can be called by any account.
     */
    fn withdraw_benefit_of(account: AccountId);

    /**
     * @notice Remove a certain beneficiary from this contract.
     * The removed beneficiary can not withdraw benefit from this contract any more.
     */
    fn terminate_benefit_of(account: AccountId);

    /**
     * @notice Withdraw a certain amount of remaining benefit to the owner.
     */
    fn withdraw_remaining_benefit(amount: Amount);
}