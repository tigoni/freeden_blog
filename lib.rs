#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod freeden_blogr {
    use ink::storage::Mapping;
    use ink::prelude::vec::Vec;


    #[derive(scale::Decode, scale::Encode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Payees{
        percentages: u128,
        amount: u128,
    }

    impl Payees {
        pub fn new() -> Self {
            Self {
                percentages: 0,
                amount: 0,
            }
        }
    }

    #[ink(storage)]
    pub struct FreedenBlogr {
        payees: Mapping<AccountId, Payees>,
        payout_amount: u128,
        account_keys: Vec<AccountId>,
    }
// Instantiate the contract with a default account and percentage
    impl FreedenBlogr {
        #[ink(constructor)]
        pub fn new() -> Self {
            let accounts = Mapping::default();
            let balance: Balance = Balance::default();
            let keys: Vec<AccountId> = Vec::new();
            Self {
                payees: accounts,
                payout_amount: balance,
                account_keys: keys
            }
        }

// Adds a payee with a percentage share to the contract 
        #[ink(message)]
        pub fn add_payee(&mut self, acc:AccountId, percentage: u128){
            let mut payee: Payees = Payees::new();
            payee.percentages = percentage; 
            self.payees.insert(acc, &payee);
        }

        //update pauout amount on new subscription
        #[ink(message)]
        pub fn add_aubscriber_amount(&mut self, amount: u128){
            self.payout_amount += amount;
        }

        #[ink(message)]
        pub fn pay_accounts(& self) {
            //distribute to accounts, the value based on the percentages
            for key in &self.account_keys {
                let mut acc = self.payees.get(key).unwrap();
                acc.amount = acc.percentages * self.payout_amount
            }

        }
        
        #[ink(message)]
        pub fn payee_balance(&self, account: AccountId) -> u128 {
            // let caller = Self::env().caller();
            self.payees.get(account).unwrap().percentages
        }


        #[ink(message)]
        pub fn total_balance(&self) -> u128 {
            self.payout_amount
        }
    }
}
