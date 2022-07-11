use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::LookupMap,
    AccountId, Balance, IntoStorageKey,
};

// pub type StorageUsage = u64;
pub type ProjectId = u64;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct OCCToken {
    /// AccountID -> Projects -> Project -> Balance.
    pub accounts: LookupMap<AccountId, LookupMap<ProjectId, Balance>>,

    /// Total supply of the all token.
    pub total_supply: Balance,
    // /// The storage size in bytes for one account.
    // pub account_storage_usage: StorageUsage,
}

impl OCCToken {
    pub fn new<S>(prefix: S) -> Self
    where
        S: IntoStorageKey,
    {
        let mut this = Self {
            accounts: LookupMap::new(prefix),
            total_supply: 0,
        };
        this
    }
}
