//! Namespace reference type.
#![cfg(feature = "no_module")]

#[cfg(feature = "no_std")]
use std::prelude::v1::*;

/// _(internals)_ A chain of [module][crate::Module] names to namespace-qualify a variable or function call.
/// Exported under the `internals` feature only.
///
/// Not available under `no_module`.
#[derive(Debug, Clone, Eq, PartialEq, Default, Hash)]
pub struct Namespace;

impl Namespace {
    /// Constant for no namespace.
    pub const NONE: Self = Self;

    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        true
    }
}
