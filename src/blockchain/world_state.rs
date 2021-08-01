/// The world state of the blockchain, which is the state after all blocks are executed
/// While not necessary (the state could be made by iterating through all blocks),
/// it's a time-saving and overhead-saving way to get a copy of the chain's most recent state
pub(crate) trait WorldState {
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