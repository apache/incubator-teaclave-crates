//! # Rhai - embedded scripting for Rust
//!
//! ![Rhai logo](https://rhai.rs/book/images/logo/rhai-banner-transparent-colour.svg)
//!
//! Rhai is a tiny, simple and fast embedded scripting language for Rust
//! that gives you a safe and easy way to add scripting to your applications.
//!
//! It provides a familiar syntax based on JavaScript+Rust and a simple Rust interface.
//!
//! # A Quick Example
//!
//! ## Contents of `my_script.rhai`
//!
//! ```rhai
//! /// Brute force factorial function
//! fn factorial(x) {
//!     if x == 1 { return 1; }
//!     x * factorial(x - 1)
//! }
//!
//! // Calling an external function 'compute'
//! compute(factorial(10))
//! ```
//!
//! ## The Rust part
//!
//! ```no_run
//! use rhai::{Engine, EvalAltResult};
//!
//! fn main() -> Result<(), Box<EvalAltResult>>
//! {
//!     // Define external function
//!     fn compute_something(x: i64) -> bool {
//!         (x % 40) == 0
//!     }
//!
//!     // Create scripting engine
//!     let mut engine = Engine::new();
//!
//!     // Register external function as 'compute'
//!     engine.register_fn("compute", compute_something);
//!
//! #   #[cfg(not(feature = "no_std"))]
//! #   #[cfg(not(target_family = "wasm"))]
//! #
//!     // Evaluate the script, expecting a 'bool' result
//!     let result: bool = engine.eval_file("my_script.rhai".into())?;
//!
//!     assert_eq!(result, true);
//!
//!     Ok(())
//! }
//! ```
//!
//! # Features
//!
#![cfg_attr(feature = "document-features", doc = document_features::document_features!(feature_label = "<span id=\"feature-{feature}\">**`{feature}`**</span>"))]
//!
//! # On-Line Documentation
//!
//! See [The Rhai Book](https://rhai.rs/book) for details on the Rhai scripting engine and language.

#![cfg_attr(feature = "no_std", no_std)]
#![deny(missing_docs)]
// #![warn(clippy::all)]
// #![warn(clippy::pedantic)]
// #![warn(clippy::nursery)]
#![warn(clippy::cargo)]
// #![warn(clippy::undocumented_unsafe_blocks)]
#![allow(clippy::unit_arg)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::used_underscore_binding)]
#![allow(clippy::inline_always)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::negative_feature_names)]
#![allow(clippy::module_inception)]
#![allow(clippy::box_collection)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::match_same_arms)]
// The lints below can be turned off to reduce signal/noise ratio
#![allow(clippy::too_many_lines)]
#![allow(clippy::let_underscore_drop)]
#![allow(clippy::absurd_extreme_comparisons)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::no_effect_underscore_binding)]
#![allow(clippy::semicolon_if_nothing_returned)]
#![allow(clippy::type_complexity)]

#[cfg(feature = "no_std")]
extern crate alloc;

#[cfg(feature = "no_std")]
extern crate no_std_compat as std;

#[cfg(feature = "no_std")]
use std::prelude::v1::*;

// Internal modules
#[macro_use]
mod reify;
#[macro_use]
mod defer;

mod api;
mod ast;
pub mod config;
mod engine;
mod eval;
mod func;
mod module;
mod optimizer;
pub mod packages;
mod parser;
#[cfg(feature = "serde")]
pub mod serde;
mod tests;
mod tokenizer;
mod types;

/// Error encountered when parsing a script.
type PERR = ParseErrorType;
/// Evaluation result.
type ERR = EvalAltResult;
/// General evaluation error for Rhai scripts.
type RhaiError = Box<ERR>;
/// Generic [`Result`] type for Rhai functions.
type RhaiResultOf<T> = Result<T, RhaiError>;
/// General [`Result`] type for Rhai functions returning [`Dynamic`] values.
type RhaiResult = RhaiResultOf<Dynamic>;

/// The system integer type. It is defined as [`i64`].
///
/// If the `only_i32` feature is enabled, this will be [`i32`] instead.
#[cfg(not(feature = "only_i32"))]
pub type INT = i64;

/// The system integer type.
/// It is defined as [`i32`] since the `only_i32` feature is used.
///
/// If the `only_i32` feature is not used, this will be `i64` instead.
#[cfg(feature = "only_i32")]
pub type INT = i32;

/// The unsigned system base integer type. It is defined as [`u64`].
///
/// If the `only_i32` feature is enabled, this will be [`u32`] instead.
#[cfg(not(feature = "only_i32"))]
#[allow(non_camel_case_types)]
type UNSIGNED_INT = u64;

/// The unsigned system integer base type.
/// It is defined as [`u32`] since the `only_i32` feature is used.
///
/// If the `only_i32` feature is not used, this will be `u64` instead.
#[cfg(feature = "only_i32")]
#[allow(non_camel_case_types)]
type UNSIGNED_INT = u32;

/// The maximum integer that can fit into a [`usize`].
#[cfg(not(target_pointer_width = "32"))]
const MAX_USIZE_INT: INT = INT::MAX;

/// The maximum integer that can fit into a [`usize`].
#[cfg(not(feature = "only_i32"))]
#[cfg(target_pointer_width = "32")]
const MAX_USIZE_INT: INT = usize::MAX as INT;

/// The maximum integer that can fit into a [`usize`].
#[cfg(feature = "only_i32")]
#[cfg(target_pointer_width = "32")]
const MAX_USIZE_INT: INT = INT::MAX;

/// Number of bits in [`INT`].
///
/// It is 64 unless the `only_i32` feature is enabled when it will be 32.
const INT_BITS: usize = std::mem::size_of::<INT>() * 8;

/// Number of bytes that make up an [`INT`].
///
/// It is 8 unless the `only_i32` feature is enabled when it will be 4.
#[cfg(not(feature = "no_index"))]
const INT_BYTES: usize = std::mem::size_of::<INT>();

/// The system floating-point type. It is defined as [`f64`].
///
/// Not available under `no_float`.
///
/// If the `f32_float` feature is enabled, this will be [`f32`] instead.
#[cfg(not(feature = "no_float"))]
#[cfg(not(feature = "f32_float"))]
pub type FLOAT = f64;

/// The system floating-point type.
/// It is defined as [`f32`] since the `f32_float` feature is used.
///
/// Not available under `no_float`.
///
/// If the `f32_float` feature is not used, this will be `f64` instead.
#[cfg(not(feature = "no_float"))]
#[cfg(feature = "f32_float")]
pub type FLOAT = f32;

/// Number of bytes that make up a [`FLOAT`].
///
/// It is 8 unless the `f32_float` feature is enabled when it will be 4.
#[cfg(not(feature = "no_float"))]
#[cfg(not(feature = "no_index"))]
const FLOAT_BYTES: usize = std::mem::size_of::<FLOAT>();

/// An exclusive integer range.
type ExclusiveRange = std::ops::Range<INT>;

/// An inclusive integer range.
type InclusiveRange = std::ops::RangeInclusive<INT>;

#[allow(deprecated)]
pub use api::build_type::{CustomType, TypeBuilder};
#[cfg(not(feature = "no_custom_syntax"))]
pub use api::custom_syntax::Expression;
#[cfg(not(feature = "no_std"))]
#[cfg(not(target_family = "wasm"))]
#[cfg(not(target_vendor = "teaclave"))]
pub use api::files::{eval_file, run_file};
pub use api::{eval::eval, events::VarDefInfo, run::run};
pub use ast::{FnAccess, AST};
use defer::Deferred;
pub use engine::{Engine, OP_CONTAINS, OP_EQUALS};
pub use eval::EvalContext;
#[cfg(not(feature = "no_function"))]
#[cfg(not(feature = "no_object"))]
use func::calc_typed_method_hash;
use func::{calc_fn_hash, calc_fn_hash_full, calc_var_hash};
pub use func::{plugin, FuncArgs, NativeCallContext, RegisterNativeFunction};
pub use module::{FnNamespace, Module};
pub use rhai_codegen::*;
#[cfg(not(feature = "no_time"))]
pub use types::Instant;
pub use types::{
    Dynamic, EvalAltResult, FnPtr, ImmutableString, LexError, ParseError, ParseErrorType, Position,
    Scope,
};

/// _(debugging)_ Module containing types for debugging.
/// Exported under the `debugging` feature only.
#[cfg(feature = "debugging")]
pub mod debugger {
    #[cfg(not(feature = "no_function"))]
    pub use super::eval::CallStackFrame;
    pub use super::eval::{BreakPoint, Debugger, DebuggerCommand, DebuggerEvent};
}

/// An identifier in Rhai. [`SmartString`](https://crates.io/crates/smartstring) is used because most
/// identifiers are ASCII and short, fewer than 23 characters, so they can be stored inline.
#[cfg(not(feature = "internals"))]
type Identifier = SmartString;

/// An identifier in Rhai. [`SmartString`](https://crates.io/crates/smartstring) is used because most
/// identifiers are ASCII and short, fewer than 23 characters, so they can be stored inline.
#[cfg(feature = "internals")]
pub type Identifier = SmartString;

/// Alias to [`Rc`][std::rc::Rc] or [`Arc`][std::sync::Arc] depending on the `sync` feature flag.
pub use func::Shared;

/// Alias to [`RefCell`][std::cell::RefCell] or [`RwLock`][std::sync::RwLock] depending on the `sync` feature flag.
pub use func::Locked;

/// A shared [`Module`].
type SharedModule = Shared<Module>;

#[cfg(not(feature = "no_function"))]
pub use func::Func;

#[cfg(not(feature = "no_function"))]
pub use ast::ScriptFnMetadata;

#[cfg(not(feature = "no_function"))]
pub use api::call_fn::CallFnOptions;

/// Variable-sized array of [`Dynamic`] values.
///
/// Not available under `no_index`.
#[cfg(not(feature = "no_index"))]
pub type Array = Vec<Dynamic>;

/// Variable-sized array of [`u8`] values (byte array).
///
/// Not available under `no_index`.
#[cfg(not(feature = "no_index"))]
pub type Blob = Vec<u8>;

/// A dictionary of [`Dynamic`] values with string keys.
///
/// Not available under `no_object`.
///
/// [`SmartString`](https://crates.io/crates/smartstring) is used as the key type because most
/// property names are ASCII and short, fewer than 23 characters, so they can be stored inline.
#[cfg(not(feature = "no_object"))]
pub type Map = std::collections::BTreeMap<Identifier, Dynamic>;

#[cfg(not(feature = "no_object"))]
pub use api::json::format_map_as_json;

#[cfg(not(feature = "no_module"))]
pub use module::ModuleResolver;

/// Module containing all built-in _module resolvers_ available to Rhai.
#[cfg(not(feature = "no_module"))]
pub use module::resolvers as module_resolvers;

#[cfg(not(feature = "no_optimize"))]
pub use optimizer::OptimizationLevel;

/// Placeholder for the optimization level.
#[cfg(feature = "no_optimize")]
pub type OptimizationLevel = ();

// Expose internal data structures.

#[cfg(feature = "internals")]
pub use types::dynamic::{AccessMode, DynamicReadLock, DynamicWriteLock, Variant};

#[cfg(feature = "internals")]
#[cfg(not(feature = "no_float"))]
pub use types::FloatWrapper;

#[cfg(feature = "internals")]
pub use types::{Span, StringsInterner};

#[cfg(feature = "internals")]
pub use tokenizer::{
    get_next_token, is_valid_function_name, is_valid_identifier, parse_string_literal, InputStream,
    MultiInputsStream, Token, TokenIterator, TokenizeState, TokenizerControl,
    TokenizerControlBlock,
};

#[cfg(feature = "internals")]
pub use parser::ParseState;

#[cfg(feature = "internals")]
pub use ast::{
    ASTFlags, ASTNode, BinaryExpr, ConditionalExpr, Expr, FlowControl, FnCallExpr, FnCallHashes,
    Ident, OpAssignment, RangeCase, ScriptFnDef, Stmt, StmtBlock, SwitchCasesCollection,
};

#[cfg(feature = "internals")]
#[cfg(not(feature = "no_custom_syntax"))]
pub use ast::CustomExpr;

#[cfg(feature = "internals")]
#[cfg(not(feature = "no_module"))]
pub use ast::Namespace;

#[cfg(feature = "internals")]
pub use func::EncapsulatedEnviron;

#[cfg(feature = "internals")]
pub use eval::{Caches, FnResolutionCache, FnResolutionCacheEntry, GlobalRuntimeState};

#[cfg(feature = "internals")]
#[allow(deprecated)]
pub use func::{locked_read, locked_write, CallableFunction, NativeCallContextStore};

#[cfg(feature = "internals")]
#[cfg(feature = "metadata")]
pub use api::definitions::Definitions;

/// Number of items to keep inline for [`StaticVec`].
const STATIC_VEC_INLINE_SIZE: usize = 3;

/// Alias to [`smallvec::SmallVec<[T; 3]>`](https://crates.io/crates/smallvec), which is a
/// specialized [`Vec`] backed by a small, inline, fixed-size array when there are ≤ 3 items stored.
///
/// # History
///
/// And Saint Attila raised the `SmallVec` up on high, saying, "O Lord, bless this Thy `SmallVec`
/// that, with it, Thou mayest blow Thine allocation costs to tiny bits in Thy mercy."
///
/// And the Lord did grin, and the people did feast upon the lambs and sloths and carp and anchovies
/// and orangutans and breakfast cereals and fruit bats and large chu...
///
/// And the Lord spake, saying, "First shalt thou depend on the [`smallvec`](https://crates.io/crates/smallvec) crate.
/// Then, shalt thou keep three inline. No more. No less. Three shalt be the number thou shalt keep inline,
/// and the number to keep inline shalt be three. Four shalt thou not keep inline, nor either keep inline
/// thou two, excepting that thou then proceed to three. Five is right out. Once the number three,
/// being the third number, be reached, then, lobbest thou thy `SmallVec` towards thy heap, who,
/// being slow and cache-naughty in My sight, shall snuff it."
///
/// # Why Three
///
/// `StaticVec` is used frequently to keep small lists of items in inline (non-heap) storage in
/// order to improve cache friendliness and reduce indirections.
///
/// The number 3, other than being the holy number, is carefully chosen for a balance between
/// storage space and reduce allocations. That is because most function calls (and most functions,
/// for that matter) contain fewer than 4 arguments, the exception being closures that capture a
/// large number of external variables.
///
/// In addition, most script blocks either contain many statements, or just one or two lines;
/// most scripts load fewer than 4 external modules; most module paths contain fewer than 4 levels
/// (e.g. `std::collections::map::HashMap` is 4 levels and it is just about as long as they get).
#[cfg(not(feature = "internals"))]
type StaticVec<T> = smallvec::SmallVec<[T; STATIC_VEC_INLINE_SIZE]>;

/// _(internals)_ Alias to [`smallvec::SmallVec<[T; 3]>`](https://crates.io/crates/smallvec),
/// which is a [`Vec`] backed by a small, inline, fixed-size array when there are ≤ 3 items stored.
/// Exported under the `internals` feature only.
///
/// # History
///
/// And Saint Attila raised the `SmallVec` up on high, saying, "O Lord, bless this Thy `SmallVec`
/// that, with it, Thou mayest blow Thine allocation costs to tiny bits in Thy mercy."
///
/// And the Lord did grin, and the people did feast upon the lambs and sloths and carp and anchovies
/// and orangutans and breakfast cereals and fruit bats and large chu...
///
/// And the Lord spake, saying, "First shalt thou depend on the [`smallvec`](https://crates.io/crates/smallvec) crate.
/// Then, shalt thou keep three inline. No more. No less. Three shalt be the number thou shalt keep inline,
/// and the number to keep inline shalt be three. Four shalt thou not keep inline, nor either keep inline
/// thou two, excepting that thou then proceed to three. Five is right out. Once the number three,
/// being the third number, be reached, then, lobbest thou thy `SmallVec` towards thy heap, who,
/// being slow and cache-naughty in My sight, shall snuff it."
///
/// # Why Three
///
/// `StaticVec` is used frequently to keep small lists of items in inline (non-heap) storage in
/// order to improve cache friendliness and reduce indirections.
///
/// The number 3, other than being the holy number, is carefully chosen for a balance between
/// storage space and reduce allocations. That is because most function calls (and most functions,
/// for that matter) contain fewer than 4 arguments, the exception being closures that capture a
/// large number of external variables.
///
/// In addition, most script blocks either contain many statements, or just one or two lines;
/// most scripts load fewer than 4 external modules; most module paths contain fewer than 4 levels
/// (e.g. `std::collections::map::HashMap` is 4 levels and it is just about as long as they get).
#[cfg(feature = "internals")]
pub type StaticVec<T> = smallvec::SmallVec<[T; STATIC_VEC_INLINE_SIZE]>;

/// Number of items to keep inline for [`FnArgsVec`].
#[cfg(not(feature = "no_closure"))]
const FN_ARGS_VEC_INLINE_SIZE: usize = 5;

/// Inline arguments storage for function calls.
///
/// # Notes
///
/// Since most usage of this is during a function call to gather up arguments, this is mostly
/// allocated on the stack, so we can tolerate a larger number of values stored inline.
///
/// Most functions have few parameters, but closures with a lot of captured variables can
/// potentially have many.  Having a larger inline storage for arguments reduces allocations in
/// scripts with heavy closure usage.
///
/// Under `no_closure`, this type aliases to [`StaticVec`][crate::StaticVec] instead.
#[cfg(not(feature = "no_closure"))]
type FnArgsVec<T> = smallvec::SmallVec<[T; FN_ARGS_VEC_INLINE_SIZE]>;

/// Inline arguments storage for function calls.
/// This type aliases to [`StaticVec`][crate::StaticVec].
#[cfg(feature = "no_closure")]
type FnArgsVec<T> = crate::StaticVec<T>;

type SmartString = smartstring::SmartString<smartstring::LazyCompact>;

// Compiler guards against mutually-exclusive feature flags

#[cfg(feature = "no_float")]
#[cfg(feature = "f32_float")]
compile_error!("`f32_float` cannot be used with `no_float`");

#[cfg(feature = "only_i32")]
#[cfg(feature = "only_i64")]
compile_error!("`only_i32` and `only_i64` cannot be used together");

#[cfg(feature = "no_std")]
#[cfg(feature = "wasm-bindgen")]
compile_error!("`wasm-bindgen` cannot be used with `no-std`");

#[cfg(feature = "no_std")]
#[cfg(feature = "stdweb")]
compile_error!("`stdweb` cannot be used with `no-std`");

#[cfg(target_family = "wasm")]
#[cfg(feature = "no_std")]
compile_error!("`no_std` cannot be used for WASM target");

#[cfg(not(target_family = "wasm"))]
#[cfg(feature = "wasm-bindgen")]
compile_error!("`wasm-bindgen` cannot be used for non-WASM target");

#[cfg(not(target_family = "wasm"))]
#[cfg(feature = "stdweb")]
compile_error!("`stdweb` cannot be used non-WASM target");

#[cfg(feature = "wasm-bindgen")]
#[cfg(feature = "stdweb")]
compile_error!("`wasm-bindgen` and `stdweb` cannot be used together");
