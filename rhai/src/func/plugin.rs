//! Module defining macros for developing _plugins_.

pub use super::CallableFunction;
use super::FnCallArgs;
pub use crate::{
    Dynamic, Engine, EvalAltResult, FnAccess, FnNamespace, ImmutableString, Module,
    NativeCallContext, Position,
};
#[cfg(feature = "no_std")]
use std::prelude::v1::*;
pub use std::{any::TypeId, mem};

/// Result of a Rhai function.
pub type RhaiResult = crate::RhaiResult;

#[cfg(not(features = "no_module"))]
pub use rhai_codegen::*;
#[cfg(features = "no_module")]
pub use rhai_codegen::{export_fn, register_exported_fn};

/// Trait implemented by a _plugin function_.
///
/// This trait should not be used directly.
/// Use the `#[export_module]` and `#[export_fn]` procedural attributes instead.
pub trait PluginFunction {
    /// Call the plugin function with the arguments provided.
    fn call(&self, context: Option<NativeCallContext>, args: &mut FnCallArgs) -> RhaiResult;

    /// Is this plugin function a method?
    #[must_use]
    fn is_method_call(&self) -> bool;

    /// Does this plugin function contain a [`NativeCallContext`] parameter?
    #[must_use]
    fn has_context(&self) -> bool;

    /// Is this plugin function pure?
    ///
    /// This defaults to `true` such that any old implementation that has constant-checking code
    /// inside the function itself will continue to work.
    #[inline(always)]
    #[must_use]
    fn is_pure(&self) -> bool {
        true
    }
}
