#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod freeden_blogr {
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;

    #[derive(scale::Decode, scale::Encode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct PayoutConfig {
        percentages: u128,
        amount: u128,
    }

    impl PayoutConfig {
        pub fn new() -> Self {
            Self {
                percentages: 0,
                amount: 0,
            }
        }
    }

    #[ink(storage)]
    pub struct FreedenBlogr {
        accounts: Mapping<AccountId, PayoutConfig>,
        subscription_total: u128,
        account_keys: Vec<AccountId>,
    }
    // Instantiate the contract with a default account and percentage
    impl FreedenBlogr {
        #[ink(constructor)]
        pub fn new() -> Self {
            let init_accounts = Mapping::default();
            let balance: Balance = Balance::default();
            let keys: Vec<AccountId> = Vec::new();
            Self {
                accounts: init_accounts,
                subscription_total: balance,
                account_keys: keys,
            }
        }

        // Adds a payee with a percentage share to the contract
        #[ink(message)]
        pub fn add_payee(&mut self, acc: AccountId, percentage: u128) {
            let mut payee: PayoutConfig = PayoutConfig::new();
            payee.percentages = percentage;
            self.accounts.insert(acc, &payee);
            //Update the keys mapper to add a key for this new payee
            self.account_keys.push(acc);
        }

        //update pauout amount on new subscription
        #[ink(message)]
        pub fn add_aubscriber_amount(&mut self, amount: u128) {
            self.subscription_total += amount;
        }

        #[ink(message)]
        pub fn pay_accounts(&mut self) {
            //distribute to accounts, the value based on the percentages
            for key in &self.account_keys {
                let acc = self.accounts.get(key);
                match acc {
                    Some(mut acc) => acc.amount = acc.percentages * self.subscription_total,
                    None => panic!("error occurred in payout calculation!!"),
                    
                }
                // acc.amount = acc.percentages * self.subscription_total
            }
        }

        #[ink(message)]
        pub fn payee_balance(&self, account: AccountId) -> u128 {
            // let caller = Self::env().caller();
            self.accounts.get(account).unwrap().amount
        }

        #[ink(message)]
        pub fn total_balance(&self) -> u128 {
            self.subscription_total
        }
    }
}
