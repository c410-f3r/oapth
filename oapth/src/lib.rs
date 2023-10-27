//! # Oapth ORM

#![allow(incomplete_features)]
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(non_lifetime_binders)]

extern crate alloc;

#[macro_use]
mod macros;

pub mod database;
mod database_ty;
mod error;
mod from_row;
mod from_rows;
#[cfg(feature = "orm")]
pub mod orm;
mod row;
#[cfg(feature = "sm")]
pub mod sm;
#[cfg(test)]
mod tests;

pub use database_ty::*;
pub use error::*;
pub use from_row::FromRow;
pub use from_rows::FromRows;
pub use row::Row;

/// The maximum number of characters that a database identifier can have. For example, tables,
/// procedures, triggers, etc.
pub type Identifier = arrayvec::ArrayString<64>;
/// Alias of [core::result::Result<T, oapth_orm::Error>].
pub type Result<T> = core::result::Result<T, Error>;
/// Used by some operations to identify different tables
pub type TableSuffix = u32;
