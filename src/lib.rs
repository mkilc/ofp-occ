use near_contract_standards::fungible_token::metadata::{FungibleTokenMetadata, FT_METADATA_SPEC};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::json_types::U128;
use near_sdk::{env, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault};
use token_impl::{OCCToken, ProjectId, TokenStorageKey};

mod metadata;
mod token;
mod token_impl;

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Metadata,
}
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    token: OCCToken,
    metadata: LazyOption<FungibleTokenMetadata>,
}

const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 288 288'%3E%3Cg id='l' data-name='l'%3E%3Cpath d='M187.58,79.81l-30.1,44.69a3.2,3.2,0,0,0,4.75,4.2L191.86,103a1.2,1.2,0,0,1,2,.91v80.46a1.2,1.2,0,0,1-2.12.77L102.18,77.93A15.35,15.35,0,0,0,90.47,72.5H87.34A15.34,15.34,0,0,0,72,87.84V201.16A15.34,15.34,0,0,0,87.34,216.5h0a15.35,15.35,0,0,0,13.08-7.31l30.1-44.69a3.2,3.2,0,0,0-4.75-4.2L96.14,186a1.2,1.2,0,0,1-2-.91V104.61a1.2,1.2,0,0,1,2.12-.77l89.55,107.23a15.35,15.35,0,0,0,11.71,5.43h3.13A15.34,15.34,0,0,0,216,201.16V87.84A15.34,15.34,0,0,0,200.66,72.5h0A15.35,15.35,0,0,0,187.58,79.81Z'/%3E%3C/g%3E%3C/svg%3E";

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new_default_meta(
        owner_id: AccountId,
        project_id: ProjectId,
        total_supply: U128,
    ) -> Self {
        Self::new(
            owner_id,
            project_id,
            total_supply,
            FungibleTokenMetadata {
                spec: FT_METADATA_SPEC.to_string(),
                name: "Example NEAR fungible token".to_string(),
                symbol: "EXAMPLE".to_string(),
                icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
                reference: None,
                reference_hash: None,
                decimals: 24,
            },
        )
    }

    #[init]
    pub fn new(
        owner_id: AccountId,
        project_id: ProjectId,
        total_supply: U128,
        metadata: FungibleTokenMetadata,
    ) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        metadata.assert_valid();

        let mut this = Self {
            token: OCCToken::new(TokenStorageKey::OccToken),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
        };

        this.token.internal_register_account(&owner_id, &project_id);
        this.token
            .internal_deposit(&owner_id, &project_id, total_supply.into());
        // near_contract_standards::fungible_token::events::FtMint {
        //     owner_id: &owner_id,
        //     amount: &total_supply,
        //     memo: Some("Initial tokens supply is minted"),
        // }
        // .emit();
        this
    }

    // fn on_account_closed(&mut self, account_id: AccountId, balance: Balance) {
    //     log!("Closed @{} with {}", account_id, balance);
    // }

    // fn on_tokens_burned(&mut self, account_id: AccountId, amount: Balance) {
    //     log!("Account @{} burned {}", account_id, amount);
    // }
}

// near_contract_standards::impl_fungible_token_core!(Contract, token, on_tokens_burned);
// near_contract_standards::impl_fungible_token_storage!(Contract, token, on_account_closed);

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env, Balance};

    use super::*;
    use crate::token::OccTokenCore;

    const TOTAL_SUPPLY: Balance = 1_000_000_000_000_000;

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    fn sample_ft_metadata() -> FungibleTokenMetadata {
        FungibleTokenMetadata {
            spec: FT_METADATA_SPEC.to_string(),
            name: String::from("Carbon Credit"),
            symbol: String::from("OCC"),
            icon: None,
            reference: None,
            reference_hash: None,
            decimals: 24,
        }
    }

    #[test]
    fn test_default_new() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let contract = Contract::new_default_meta(accounts(1).into(), 1, TOTAL_SUPPLY.into());
        testing_env!(context.is_view(true).build());

        assert_eq!(contract.token.ft_total_supply(), TOTAL_SUPPLY);
        assert_eq!(
            contract.token.ft_balance_of_project(accounts(1).into(), 1),
            TOTAL_SUPPLY
        );
    }

    #[test]
    fn test_new() {
        let mut context = get_context(accounts(2));
        testing_env!(context.build());
        let contract = Contract::new(
            accounts(2).into(),
            2,
            TOTAL_SUPPLY.into(),
            sample_ft_metadata(),
        );
        testing_env!(context.is_view(true).build());
        assert_eq!(contract.token.ft_total_supply(), TOTAL_SUPPLY);
        assert_eq!(
            contract.token.ft_balance_of_project(accounts(2).into(), 2),
            TOTAL_SUPPLY
        );
        // assert_eq!(contract.ft_metadata().name, sample_ft_metadata().name)
    }

    #[test]
    #[should_panic(expected = "The contract is not initialized")]
    fn test_default() {
        let context = get_context(accounts(3));
        testing_env!(context.build());
        let _contract = Contract::default();
    }
}
