use near_sdk::AccountId;

pub trait OccTokenCore {
    fn ft_total_supply(&self) -> u128;

    fn ft_balance_of_project(&self, account_id: AccountId, project_id: u64) -> u128;
}
