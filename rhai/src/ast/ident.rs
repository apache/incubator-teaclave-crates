//! Module defining script identifiers.

use crate::{ImmutableString, Position};
#[cfg(feature = "no_std")]
use std::prelude::v1::*;
use std::{
    borrow::Borrow,
    fmt,
    hash::Hash,
    ops::{Deref, DerefMut},
};

/// _(internals)_ An identifier containing a name and a [position][Position].
/// Exported under the `internals` feature only.
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Ident {
    /// Identifier name.
    pub name: ImmutableString,
    /// Position.
    pub pos: Position,
}

impl fmt::Debug for Ident {
    #[cold]
    #[inline(never)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.name)?;
        self.pos.debug_print(f)
    }
}

impl Borrow<str> for Ident {
    #[inline(always)]
    #[must_use]
    fn borrow(&self) -> &str {
        self.name.as_ref()
    }
}

impl AsRef<str> for Ident {
    #[inline(always)]
    #[must_use]
    fn as_ref(&self) -> &str {
        self.name.as_ref()
    }
}

impl Deref for Ident {
    type Target = ImmutableString;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.name
    }
}

impl DerefMut for Ident {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.name
    }
}

impl Ident {
    /// Get the name of the identifier as a string slice.
    #[inline(always)]
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.name.as_str()
    }
}
