//! Placeholder settings for [`Engine`]'s limitations.
#![cfg(feature = "unchecked")]

use crate::Engine;

impl Engine {
    /// The maximum levels of function calls allowed for a script.
    ///
    /// Always returns [`usize::MAX`].
    #[inline(always)]
    #[must_use]
    pub const fn max_call_levels(&self) -> usize {
        usize::MAX
    }
    /// The maximum number of operations allowed for a script to run (0 for unlimited).
    ///
    /// Always returns zero.
    #[inline(always)]
    #[must_use]
    pub const fn max_operations(&self) -> u64 {
        0
    }
    /// The maximum number of imported [modules][crate::Module] allowed for a script.
    ///
    /// Always returns [`usize::MAX`].
    #[inline(always)]
    #[must_use]
    pub const fn max_modules(&self) -> usize {
        usize::MAX
    }
    /// The depth limit for expressions (0 for unlimited).
    ///
    /// Always returns zero.
    #[inline(always)]
    #[must_use]
    pub const fn max_expr_depth(&self) -> usize {
        0
    }
    /// The depth limit for expressions in functions (0 for unlimited).
    ///
    /// Always returns zero.
    #[inline(always)]
    #[must_use]
    pub const fn max_function_expr_depth(&self) -> usize {
        0
    }
    /// The maximum length of [strings][crate::ImmutableString] (0 for unlimited).
    ///
    /// Always returns zero.
    #[inline(always)]
    #[must_use]
    pub const fn max_string_size(&self) -> usize {
        0
    }
    /// The maximum length of [arrays][crate::Array] (0 for unlimited).
    ///
    /// Always returns zero.
    #[inline(always)]
    #[must_use]
    pub const fn max_array_size(&self) -> usize {
        0
    }
    /// The maximum size of [object maps][crate::Map] (0 for unlimited).
    ///
    /// Always returns zero.
    #[inline(always)]
    #[must_use]
    pub const fn max_map_size(&self) -> usize {
        0
    }
}
