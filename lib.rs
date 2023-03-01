#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod freeden_blogr {
    use ink::storage::Mapping;


    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Payees{
        account: AccountId,
        percentages: u128,
    }

    #[ink(storage)]
    pub struct FreedenBlogr {
        payees: Mapping<AccountId, u128>,
        payout_amount: u128,
    }
// Instantiate the contract with a default account and percentage
    impl FreedenBlogr {
        #[ink(constructor)]
        pub fn new() -> Self {
            let accounts = Mapping::default();
            let balance: Balance = Balance::default();
            Self {
                payees: accounts,
                payout_amount: balance,
            }
        }

// Adds a payee with a percentage share to the contract 
        #[ink(message)]
        pub fn add_payee(&mut self, payee:AccountId, percentage: u128){
            self.payees.insert(payee, &percentage);
        }

        //update pauout amount on new subscription
        #[ink(message)]
        pub fn add_aubscriber_amount(&mut self, amount: u128){
            self.payout_amount += amount;
        }

        #[ink(message)]
        pub fn pay_accounts(&mut self) {
            //distribute to accounts, the value based on the percentages
        }
        
        #[ink(message)]
        pub fn payee_balance(&self, account: AccountId) -> u128 {
            // let caller = Self::env().caller();
            self.payees.get(&account).unwrap_or_default()
        }


        #[ink(message)]
        pub fn total_balance(&self) -> u128 {
            self.payout_amount
        }
    }
}
