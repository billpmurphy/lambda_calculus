//! [Church-encoded](https://en.wikipedia.org/wiki/Church_encoding) data and operators
//!
//! This module can optionally not be built using `features = ["no_church"]`.

pub mod numerals;
pub mod booleans;
pub mod pairs;
pub mod lists;

/// An error that can be returned when a method intended for Church-encoded data is applied to
/// an inapplicable `Term`.
#[derive(Debug, PartialEq)]
pub enum ChurchError {
    /// not a Church number
    NotANum,
    /// not a Church pair
    NotAPair,
    /// not a Church list
    NotAList
}
