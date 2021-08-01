use crate::core::transaction::Transaction;

/// A single block in the chain
/// Contains a list of transactions
pub struct Block {
    /// Transactions the block includes (must be at least 1)
    pub(crate) transactions: Vec<Transaction>,

    /// Hash of the previous block (necessary if not genesis)
    prev_hash: Option<String>,

    /// Hash of the current block
    hash: Option<String>,

    /// Nonce for PoW
    nonce: u128
}