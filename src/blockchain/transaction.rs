/// A request to modify the blockchain WorldState
pub struct Transaction {
    /// A unique number (prevents replay attacks)
    nonce: u128,
    /// Account ID of the caller
    from: String,
    /// Timestamp
    created_at: SystemTime,
    /// Type of transaction and additional info
    pub(crate) record: TransactionData,
    /// Signature of the message
    signature: Option<String>
}