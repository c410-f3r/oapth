use crate::BackEndGeneric;

/// Back end is the bridge between Rust and a database.
pub trait BackEnd: BackEndGeneric {}
