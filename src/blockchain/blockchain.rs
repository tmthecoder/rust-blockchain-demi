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