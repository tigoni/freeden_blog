#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod freeden_blogr {
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;
    use scale::CompactAs;
    use sp_arithmetic::fixed_point::FixedU128;

    #[derive(scale::Decode, scale::Encode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct PayoutConfig {
        percentages: u128,
    }

    impl PayoutConfig {
        pub fn new() -> Self {
            Self {
                percentages: 0,
            }
        }
    }

    #[ink(storage)]
    pub struct FreedenBlogr {
        accounts: Mapping<AccountId, PayoutConfig>,
        account_keys: Vec<AccountId>,
        subscribers: Mapping<AccountId, Balance>,
    }
    // Instantiate the contract with a default account and percentage
    impl FreedenBlogr {
        #[ink(constructor)]
        pub fn new() -> Self {
            let init_accounts = Mapping::default();
            let keys: Vec<AccountId> = Vec::new();
            Self {
                accounts: init_accounts,
                account_keys: keys,
                subscribers: Mapping::default(),
            }
        }

        // Adds a payee with a percentage share to the contract 
        #[ink(message)]
        pub fn add_payee(&mut self, acc: AccountId, percentage: FixedU128) {
            let mut payee: PayoutConfig = PayoutConfig::new();
            payee.percentages = *percentage.encode_as();
            self.accounts.insert(acc, &payee);
            self.account_keys.push(acc);
        }

        #[ink(message, payable)]
        pub fn subscribe(&mut self) {
            let caller = Self::env().caller();
            let payment = Self::env().transferred_value();
            self.subscribers.insert(caller,&payment);
        }


        #[ink(message)]
        pub fn pay_accounts(&mut self) {
            for key in &self.account_keys {
                let acc = self.accounts.get(key);
                match acc {
                    Some(acc) => {
                        let pay_alloc = FixedU128::decode_from(acc.percentages).unwrap_or_default()
                            * FixedU128::from(Self::env().balance());
                            let amount: Balance = acc.percentages / 100 * Self::env().balance();
                            Self::env().transfer(*key, amount).unwrap_or_default();
                    }
                    None => panic!("error occurred in payout calculation!!"),
                }
            }
        }

        #[ink(message)]
        pub fn total_balance(&self) -> Balance {
            Self::env().balance()
        }
    }
}
