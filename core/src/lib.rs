#![cfg_attr(docsrs, feature(doc_cfg))]
#![no_std]
#![warn(missing_docs)]
#![doc = include_str!("../README.md")] // the doc is in another castle

extern crate self as peer_pressure;

mod ops;
mod tuple;

#[cfg(feature = "async")]
mod resolve;

#[cfg(test)]
mod tests;

use core::convert::Infallible;

pub use ops::*;
#[cfg(feature = "derive")]
pub use peer_pressure_derive as derive;
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub use resolve::{Resolve, ResolveFrom};

/// Marker trait for types that are valid by construction.
///
/// Every [Valid] type trivially validates as itself via [Validate] and [ValidateFrom].
///
/// # Examples
///
/// ```
/// use peer_pressure::{Valid, Validate};
///
/// #[derive(Clone, Debug, PartialEq)]
/// pub struct NonEmptyVec<T>(Vec<T>);
///
/// impl<T> NonEmptyVec<T> {
///     pub fn new(inner: Vec<T>) -> Option<Self> {
///         (inner.len() > 0).then_some(Self(inner))
///     }
/// }
///
/// // `NonEmptyVec` can only be constructed via `NonEmptyVec::new` which ensures the non-emptiness
/// // invariant holds for all `NonEmptyVec` values.
/// impl<T> Valid for NonEmptyVec<T> {}
///
/// let p = NonEmptyVec::new(vec![1,2,3]).unwrap();
/// assert_eq!(p.clone().validate(), Ok(p));
/// ```
pub trait Valid {}

/// Deterministic validation.
///
/// Types implementing [Validate] commit to a single, canonical notion of validity, encoded
/// by the `Output` type. The implementation of [Validate] must ensure that all invariants
/// assumed of `Output` hold, and `Output` itself should not be constructible in a way that would
/// violate those invariants.
///
/// In other words, `Output` should correctly implement [Valid].
///
/// # Examples
///
/// ```rust
/// use peer_pressure::Validate;
///
/// struct RawAge(i32);
///
/// struct Age(u32);
///
/// impl Validate for RawAge {
///     // Age being non-negative is a context-free property
///     type Context = ();
///     type Output = Age;
///     type Error = &'static str;
///
///     fn validate_in_context(self, _: &()) -> Result<Age, &'static str> {
///         if self.0 >= 0 {
///             Ok(Age(self.0 as u32))
///         } else {
///             Err("negative age")
///         }
///     }
/// }
///
/// let age = RawAge(30).validate().unwrap();
/// assert_eq!(age.0, 30);
/// assert!(RawAge(-1).validate().is_err());
/// ```
///
/// ```rust
/// use peer_pressure::Validate;
///
/// type ScrabbleHand = Vec<char>;
/// pub struct Word(String);
///
/// pub struct LegalWord(String);
///
/// impl Validate for Word {
///     // A scrabble word is valid for a given set of letters on hand
///     type Context = ScrabbleHand;
///     type Output = LegalWord;
///     type Error = &'static str;
///
///     fn validate_in_context(self, hand: &ScrabbleHand) -> Result<LegalWord, Self::Error> {
///         todo!("cmon, you can implement that yourself")
///     }
/// }
/// ```
pub trait Validate: Sized {
    /// Immutable context of the validation.
    ///
    /// Use `()` for context-free checks.
    type Context: ?Sized;
    /// Validated form of `Self`.
    type Output;
    /// Validation error
    type Error;

    /// Validates `self` in given context, returning either a valid `Output` or `Error`.
    fn validate_in_context(self, ctx: &Self::Context) -> Result<Self::Output, Self::Error>;

    /// Validates `self`, returning either a valid `Output` or `Error`.
    ///
    /// Available only for types with context-free validation (ie. for which `Context` is equivalent to `()`).
    fn validate(self) -> Result<Self::Output, Self::Error>
    where
        Self::Context: From<()>,
    {
        self.validate_in_context(&().into())
    }
}

impl<T: Valid> Validate for T {
    type Context = ();
    type Output = T;
    type Error = Infallible;

    fn validate_in_context(self, _: &()) -> Result<T, Infallible> {
        Ok(self)
    }
}

/// Target-side dual of [Validate].
///
/// Unlike [Validate], which forces a single validated output type, [ValidateFrom] allows
/// the same type to be validated from multiple source types, since many domain types may
/// arrive in various formats. This should not be abused to make a single raw value validate
/// into multiple types, but I can't force you.
///
/// # Examples
///
/// ```
/// use peer_pressure::ValidateFrom;
///
/// struct Email(String);
///
/// impl ValidateFrom<&str> for Email {
///     type Context = ();
///     type Error = &'static str;
///
///     fn validate_from_in_context(raw: &str, _: &()) -> Result<Self, Self::Error> {
///         if raw.contains('@') { Ok(Email(raw.to_string())) } else { Err("missing @") }
///     }
/// }
///
/// assert!(Email::validate_from("alice@example.com").is_ok());
/// assert!(Email::validate_from("not-an-email").is_err());
/// ```
pub trait ValidateFrom<Raw>: Sized {
    /// Immutable context of the validation.
    ///
    /// Use `()` for context-free checks.
    type Context: ?Sized;
    /// Validation error
    type Error;

    /// Validates `raw` in given context, returning either a valid `Self` or `Error`.
    fn validate_from_in_context(raw: Raw, ctx: &Self::Context) -> Result<Self, Self::Error>;

    /// Validates `raw`, returning either a valid `Self` or `Error`.
    ///
    /// Available only for types with context-free validation (ie. for which `Context = ()`).
    fn validate_from(raw: Raw) -> Result<Self, Self::Error>
    where
        Self::Context: From<()>,
    {
        Self::validate_from_in_context(raw, &().into())
    }
}

impl<T: Valid, R: Into<T>> ValidateFrom<R> for T {
    type Context = ();
    type Error = Infallible;

    fn validate_from_in_context(raw: R, _: &()) -> Result<T, Infallible> {
        Ok(raw.into())
    }
}
