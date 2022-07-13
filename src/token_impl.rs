use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::LookupMap,
    env, AccountId, Balance, BorshStorageKey, IntoStorageKey,
};

use crate::token::OccTokenCore;

pub type StorageUsage = u64;
pub type ProjectId = u64;
pub type Projects = LookupMap<ProjectId, Balance>;

#[derive(BorshSerialize, BorshStorageKey)]
pub enum TokenStorageKey {
    OccToken,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct OCCToken {
    /// AccountID -> Projects -> Project -> Balance.
    pub accounts: LookupMap<AccountId, Projects>,

    /// Total supply of the all token.
    pub total_supply: Balance,

    /// The storage size in bytes for one account.
    pub account_storage_usage: StorageUsage,
}

impl OCCToken {
    pub fn new<S>(prefix: S) -> Self
    where
        S: IntoStorageKey,
    {
        let mut this = Self {
            accounts: LookupMap::new(prefix),
            total_supply: 0,
            account_storage_usage: 0,
        };
        this.measure_account_storage_usage();
        this
    }

    fn measure_account_storage_usage(&mut self) {
        let initial_storage_usage = env::storage_usage();

        let tmp_account_id = AccountId::new_unchecked("a".repeat(64));
        let tmp_project_id: u64 = 1;
        let tmp_project_balance: u128 = 10000000;
        let mut tmp_projects: LookupMap<u64, u128> = LookupMap::new(TokenStorageKey::OccToken);
        tmp_projects.insert(&tmp_project_id, &tmp_project_balance);

        self.accounts.insert(&tmp_account_id, &tmp_projects);
        self.account_storage_usage = env::storage_usage() - initial_storage_usage;
        self.accounts.remove(&tmp_account_id);
    }

    pub fn internal_unwrap_balance(&self, projects: &Projects, project_id: &ProjectId) -> Balance {
        match projects.get(project_id) {
            Some(balance) => balance,
            None => {
                env::panic_str(format!("The project {} is not registered", &project_id).as_str())
            }
        }
    }

    pub fn internal_unwrap_projects_of(&self, account_id: &AccountId) -> Projects {
        match self.accounts.get(account_id) {
            Some(projects) => projects,
            None => {
                env::panic_str(format!("The account {} is not registered", &account_id).as_str())
            }
        }
    }

    // todo
    pub fn internal_deposit(
        &mut self,
        account_id: &AccountId,
        project_id: &ProjectId,
        amount: Balance,
    ) {
        let mut project_by_account = self.internal_unwrap_projects_of(account_id);
        let old_balance_by_project = self.internal_unwrap_balance(&project_by_account, project_id);

        if let Some(new_balance) = old_balance_by_project.checked_add(amount) {
            project_by_account.insert(project_id, &new_balance);

            self.total_supply = self
                .total_supply
                .checked_add(amount)
                .unwrap_or_else(|| env::panic_str("Total supply overflow"));
        } else {
            env::panic_str("Balance overflow");
        }
    }

    pub fn internal_register_account(&mut self, account_id: &AccountId, project_id: &ProjectId) {
        let mut projects: Projects = LookupMap::new(TokenStorageKey::OccToken);
        projects.insert(project_id, &0);

        if self.accounts.insert(account_id, &projects).is_some() {
            env::panic_str("The account is already registered");
        }
    }
}

impl OccTokenCore for OCCToken {
    fn ft_total_supply(&self) -> u128 {
        self.total_supply.into()
    }

    fn ft_balance_of_project(&self, account_id: AccountId, project_id: ProjectId) -> u128 {
        let projects = self
            .accounts
            .get(&account_id)
            .unwrap_or(LookupMap::new(TokenStorageKey::OccToken));

        projects.get(&project_id).unwrap_or(0)
    }
}
