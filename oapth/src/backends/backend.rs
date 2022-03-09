use crate::BackendGeneric;

/// Back end is the bridge between Rust and a database.
pub trait Backend: BackendGeneric {}
