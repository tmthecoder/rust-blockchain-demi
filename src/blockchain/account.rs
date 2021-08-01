/// An account on the Blockchain
/// The basic part of the world state
pub struct Account {
    /// A key value store for the account
    store: HashMap<String, String>,

    /// The account type (user or something else)
    acc_type: AccountType,

    /// Amount of tokens the account holds
    tokens: u128,
}

/// The type of account used
/// Different types could represent different system roles
pub enum AccountType {
    /// A normal user
    User
}