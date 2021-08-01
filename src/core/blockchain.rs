use crate::core::block::Block;
use std::collections::HashMap;
use crate::core::account::{Account, AccountType};
use crate::core::transaction::Transaction;

/// The whole blockchain
pub struct Blockchain {
    /// All of the accepted or valid blocks in the chain
    pub blocks: Vec<Block>,

    /// Map containing accounts with their correlating accountId
    /// Also represents the WorldState
    pub accounts: HashMap<String, Account>,

    /// Stores transactions that haven't been added to the chain yet
    pending_transactions: Vec<Transaction>
}

/// The world state of the blockchain, which is the state after all blocks are executed
/// While not necessary (the state could be made by iterating through all blocks),
/// it's a time-saving and overhead-saving way to get a copy of the chain's most recent state
trait WorldState {
    /// Gets all registered user ids
    fn get_user_ids(&self) -> Vec<String>;

    /// Returns a mutable version of the account of the given id
    fn get_account_by_id_mut(&mut self, id: &String) -> Option<&mut Account>;

    /// Returns the account of the given id
    fn get_account_by_id(&self, id: &String) -> Option<&Account>;

    /// Creates a new account
    fn create_account(&mut self, id: String, account_type: AccountType)
                      -> Result<(), &'static str>;
}