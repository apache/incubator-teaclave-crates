//! Module defining the standard Rhai function type.

use super::native::{FnAny, FnPlugin, IteratorFn, SendSync};
use crate::ast::FnAccess;
use crate::plugin::PluginFunction;
use crate::Shared;
use std::fmt;
#[cfg(feature = "no_std")]
use std::prelude::v1::*;

/// _(internals)_ Encapsulated AST environment.
/// Exported under the `internals` feature only.
///
/// 1) functions defined within the same AST
/// 2) the stack of imported [modules][crate::Module]
/// 3) global constants
#[derive(Debug, Clone)]
pub struct EncapsulatedEnviron {
    /// Functions defined within the same [`AST`][crate::AST].
    #[cfg(not(feature = "no_function"))]
    pub lib: crate::SharedModule,
    /// Imported [modules][crate::Module].
    #[cfg(not(feature = "no_module"))]
    pub imports: Box<crate::StaticVec<(crate::ImmutableString, crate::SharedModule)>>,
    /// Globally-defined constants.
    #[cfg(not(feature = "no_module"))]
    #[cfg(not(feature = "no_function"))]
    pub constants: Option<crate::eval::SharedGlobalConstants>,
}

/// _(internals)_ A type encapsulating a function callable by Rhai.
/// Exported under the `internals` feature only.
#[derive(Clone)]
#[non_exhaustive]
pub enum CallableFunction {
    /// A pure native Rust function with all arguments passed by value.
    Pure {
        /// Shared function pointer.
        func: Shared<FnAny>,
        /// Does the function take a [`NativeCallContext`][crate::NativeCallContext] parameter?
        has_context: bool,
        /// This is a dummy field and is not used.
        is_pure: bool,
    },
    /// A native Rust object method with the first argument passed by reference,
    /// and the rest passed by value.
    Method {
        /// Shared function pointer.
        func: Shared<FnAny>,
        /// Does the function take a [`NativeCallContext`][crate::NativeCallContext] parameter?
        has_context: bool,
        /// Allow operating on constants?
        is_pure: bool,
    },
    /// An iterator function.
    Iterator {
        /// Shared function pointer.
        func: Shared<IteratorFn>,
    },
    /// A plugin function,
    Plugin {
        /// Shared function pointer.
        func: Shared<FnPlugin>,
    },
    /// A script-defined function.
    #[cfg(not(feature = "no_function"))]
    Script {
        /// Shared reference to the [`ScriptFnDef`][crate::ast::ScriptFnDef] function definition.
        fn_def: Shared<crate::ast::ScriptFnDef>,
        /// Encapsulated environment, if any.
        environ: Option<Shared<EncapsulatedEnviron>>,
    },
}

impl fmt::Debug for CallableFunction {
    #[cold]
    #[inline(never)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pure { .. } => f.write_str("NativePureFunction"),
            Self::Method { .. } => f.write_str("NativeMethod"),
            Self::Iterator { .. } => f.write_str("NativeIterator"),
            Self::Plugin { .. } => f.write_str("PluginFunction"),

            #[cfg(not(feature = "no_function"))]
            Self::Script { fn_def, .. } => fmt::Debug::fmt(fn_def, f),
        }
    }
}

impl fmt::Display for CallableFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pure { .. } => f.write_str("NativePureFunction"),
            Self::Method { .. } => f.write_str("NativeMethod"),
            Self::Iterator { .. } => f.write_str("NativeIterator"),
            Self::Plugin { .. } => f.write_str("PluginFunction"),

            #[cfg(not(feature = "no_function"))]
            Self::Script { fn_def, .. } => fmt::Display::fmt(fn_def, f),
        }
    }
}

impl CallableFunction {
    /// Is this a pure native Rust function?
    #[inline]
    #[must_use]
    pub fn is_pure(&self) -> bool {
        match self {
            Self::Pure { .. } => true,
            Self::Method { is_pure, .. } => *is_pure,
            Self::Iterator { .. } => true,

            Self::Plugin { func, .. } => func.is_pure(),

            #[cfg(not(feature = "no_function"))]
            Self::Script { .. } => false,
        }
    }
    /// Is this a native Rust method function?
    #[inline]
    #[must_use]
    pub fn is_method(&self) -> bool {
        match self {
            Self::Method { .. } => true,
            Self::Pure { .. } | Self::Iterator { .. } => false,

            Self::Plugin { func, .. } => func.is_method_call(),

            #[cfg(not(feature = "no_function"))]
            Self::Script { .. } => false,
        }
    }
    /// Is this an iterator function?
    #[inline]
    #[must_use]
    pub const fn is_iter(&self) -> bool {
        match self {
            Self::Iterator { .. } => true,
            Self::Pure { .. } | Self::Method { .. } | Self::Plugin { .. } => false,

            #[cfg(not(feature = "no_function"))]
            Self::Script { .. } => false,
        }
    }
    /// Is this a script-defined function?
    #[inline]
    #[must_use]
    pub const fn is_script(&self) -> bool {
        #[cfg(feature = "no_function")]
        return false;

        #[cfg(not(feature = "no_function"))]
        match self {
            Self::Script { .. } => true,
            Self::Pure { .. }
            | Self::Method { .. }
            | Self::Iterator { .. }
            | Self::Plugin { .. } => false,
        }
    }
    /// Is this a plugin function?
    #[inline]
    #[must_use]
    pub const fn is_plugin_fn(&self) -> bool {
        match self {
            Self::Plugin { .. } => true,
            Self::Pure { .. } | Self::Method { .. } | Self::Iterator { .. } => false,

            #[cfg(not(feature = "no_function"))]
            Self::Script { .. } => false,
        }
    }
    /// Is this a native Rust function?
    #[inline]
    #[must_use]
    pub const fn is_native(&self) -> bool {
        #[cfg(feature = "no_function")]
        return true;

        #[cfg(not(feature = "no_function"))]
        match self {
            Self::Pure { .. }
            | Self::Method { .. }
            | Self::Plugin { .. }
            | Self::Iterator { .. } => true,
            Self::Script { .. } => false,
        }
    }
    /// Is there a [`NativeCallContext`][crate::NativeCallContext] parameter?
    #[inline]
    #[must_use]
    pub fn has_context(&self) -> bool {
        match self {
            Self::Pure { has_context, .. } | Self::Method { has_context, .. } => *has_context,
            Self::Plugin { func, .. } => func.has_context(),
            Self::Iterator { .. } => false,
            #[cfg(not(feature = "no_function"))]
            Self::Script { .. } => false,
        }
    }
    /// Get the access mode.
    #[inline]
    #[must_use]
    pub fn access(&self) -> FnAccess {
        #[cfg(feature = "no_function")]
        return FnAccess::Public;

        #[cfg(not(feature = "no_function"))]
        match self {
            Self::Plugin { .. }
            | Self::Pure { .. }
            | Self::Method { .. }
            | Self::Iterator { .. } => FnAccess::Public,
            Self::Script { fn_def, .. } => fn_def.access,
        }
    }
    /// Get a shared reference to a native Rust function.
    #[inline]
    #[must_use]
    pub fn get_native_fn(&self) -> Option<&Shared<FnAny>> {
        match self {
            Self::Pure { func, .. } | Self::Method { func, .. } => Some(func),
            Self::Iterator { .. } | Self::Plugin { .. } => None,

            #[cfg(not(feature = "no_function"))]
            Self::Script { .. } => None,
        }
    }
    /// Get a shared reference to a script-defined function definition.
    ///
    /// Not available under `no_function`.
    #[cfg(not(feature = "no_function"))]
    #[inline]
    #[must_use]
    pub const fn get_script_fn_def(&self) -> Option<&Shared<crate::ast::ScriptFnDef>> {
        match self {
            Self::Pure { .. }
            | Self::Method { .. }
            | Self::Iterator { .. }
            | Self::Plugin { .. } => None,
            Self::Script { fn_def, .. } => Some(fn_def),
        }
    }
    /// Get a reference to the shared encapsulated environment of the function definition.
    ///
    /// Not available under `no_function` or `no_module`.
    #[inline]
    #[must_use]
    pub fn get_encapsulated_environ(&self) -> Option<&EncapsulatedEnviron> {
        match self {
            Self::Pure { .. }
            | Self::Method { .. }
            | Self::Iterator { .. }
            | Self::Plugin { .. } => None,

            #[cfg(not(feature = "no_function"))]
            Self::Script { environ, .. } => environ.as_deref(),
        }
    }
    /// Get a reference to an iterator function.
    #[inline]
    #[must_use]
    pub fn get_iter_fn(&self) -> Option<&IteratorFn> {
        match self {
            Self::Iterator { func, .. } => Some(&**func),
            Self::Pure { .. } | Self::Method { .. } | Self::Plugin { .. } => None,

            #[cfg(not(feature = "no_function"))]
            Self::Script { .. } => None,
        }
    }
    /// Get a shared reference to a plugin function.
    #[inline]
    #[must_use]
    pub fn get_plugin_fn(&self) -> Option<&Shared<FnPlugin>> {
        match self {
            Self::Plugin { func, .. } => Some(func),
            Self::Pure { .. } | Self::Method { .. } | Self::Iterator { .. } => None,

            #[cfg(not(feature = "no_function"))]
            Self::Script { .. } => None,
        }
    }
}

#[cfg(not(feature = "no_function"))]
impl From<crate::ast::ScriptFnDef> for CallableFunction {
    #[inline(always)]
    fn from(fn_def: crate::ast::ScriptFnDef) -> Self {
        Self::Script {
            fn_def: fn_def.into(),
            environ: None,
        }
    }
}

#[cfg(not(feature = "no_function"))]
impl From<Shared<crate::ast::ScriptFnDef>> for CallableFunction {
    #[inline(always)]
    fn from(fn_def: Shared<crate::ast::ScriptFnDef>) -> Self {
        Self::Script {
            fn_def,
            environ: None,
        }
    }
}

impl<T: PluginFunction + 'static + SendSync> From<T> for CallableFunction {
    #[inline(always)]
    fn from(func: T) -> Self {
        Self::Plugin {
            func: Shared::new(func),
        }
    }
}

impl From<Shared<FnPlugin>> for CallableFunction {
    #[inline(always)]
    fn from(func: Shared<FnPlugin>) -> Self {
        Self::Plugin { func }
    }
}
