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

/// A single operation on the chain
/// Uses rust enums and their data storage capacities
pub enum TransactionData {
    /// Create a new user account
    CreateUserAccount(String),
    /// Change any stored value in an account
    ChangeStoreValue{key: String, value: String},
    /// Move tokens from one account to another
    TransferTokens{to: String, amount: u128},
    /// Create tokens out of the blue
    CreateTokens{receiver: String, amount: u128}
    // Add more as needed
}