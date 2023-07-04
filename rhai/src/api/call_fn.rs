//! Module that defines the `call_fn` API of [`Engine`].
#![cfg(not(feature = "no_function"))]

use crate::eval::{Caches, GlobalRuntimeState};
use crate::types::dynamic::Variant;
use crate::{
    Dynamic, Engine, FuncArgs, Position, RhaiResult, RhaiResultOf, Scope, StaticVec, AST, ERR,
};
#[cfg(feature = "no_std")]
use std::prelude::v1::*;
use std::{any::type_name, mem};

/// Options for calling a script-defined function via [`Engine::call_fn_with_options`].
#[derive(Debug, Hash)]
#[non_exhaustive]
#[must_use]
pub struct CallFnOptions<'t> {
    /// A value for binding to the `this` pointer (if any). Default [`None`].
    pub this_ptr: Option<&'t mut Dynamic>,
    /// The custom state of this evaluation run (if any), overrides [`Engine::default_tag`]. Default [`None`].
    pub tag: Option<Dynamic>,
    /// Evaluate the [`AST`] to load necessary modules before calling the function? Default `true`.
    pub eval_ast: bool,
    /// Rewind the [`Scope`] after the function call? Default `true`.
    pub rewind_scope: bool,
}

impl Default for CallFnOptions<'_> {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> CallFnOptions<'a> {
    /// Create a default [`CallFnOptions`].
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            this_ptr: None,
            tag: None,
            eval_ast: true,
            rewind_scope: true,
        }
    }
    /// Bind to the `this` pointer.
    #[inline(always)]
    pub fn bind_this_ptr(mut self, value: &'a mut Dynamic) -> Self {
        self.this_ptr = Some(value);
        self
    }
    /// Set the custom state of this evaluation run (if any).
    #[inline(always)]
    pub fn with_tag(mut self, value: impl Variant + Clone) -> Self {
        self.tag = Some(Dynamic::from(value));
        self
    }
    /// Set whether to evaluate the [`AST`] to load necessary modules before calling the function.
    #[inline(always)]
    pub const fn eval_ast(mut self, value: bool) -> Self {
        self.eval_ast = value;
        self
    }
    /// Set whether to rewind the [`Scope`] after the function call.
    #[inline(always)]
    pub const fn rewind_scope(mut self, value: bool) -> Self {
        self.rewind_scope = value;
        self
    }
}

impl Engine {
    /// Call a script function defined in an [`AST`] with multiple arguments.
    ///
    /// Not available under `no_function`.
    ///
    /// The [`AST`] is evaluated before calling the function.
    /// This allows a script to load the necessary modules.
    /// This is usually desired. If not, use [`call_fn_with_options`][Engine::call_fn_with_options] instead.
    ///
    /// # Example
    ///
    /// ```
    /// # fn main() -> Result<(), Box<rhai::EvalAltResult>> {
    /// use rhai::{Engine, Scope};
    ///
    /// let engine = Engine::new();
    ///
    /// let ast = engine.compile("
    ///     fn add(x, y) { len(x) + y + foo }
    ///     fn add1(x)   { len(x) + 1 + foo }
    ///     fn bar()     { foo/2 }
    /// ")?;
    ///
    /// let mut scope = Scope::new();
    /// scope.push("foo", 42_i64);
    ///
    /// // Call the script-defined function
    /// let result = engine.call_fn::<i64>(&mut scope, &ast, "add", ( "abc", 123_i64 ) )?;
    /// assert_eq!(result, 168);
    ///
    /// let result = engine.call_fn::<i64>(&mut scope, &ast, "add1", ( "abc", ) )?;
    /// //                                                           ^^^^^^^^^^ tuple of one
    /// assert_eq!(result, 46);
    ///
    /// let result = engine.call_fn::<i64>(&mut scope, &ast, "bar", () )?;
    /// assert_eq!(result, 21);
    /// # Ok(())
    /// # }
    /// ```
    #[inline(always)]
    pub fn call_fn<T: Variant + Clone>(
        &self,
        scope: &mut Scope,
        ast: &AST,
        name: impl AsRef<str>,
        args: impl FuncArgs,
    ) -> RhaiResultOf<T> {
        self.call_fn_with_options(CallFnOptions::default(), scope, ast, name, args)
    }
    /// Call a script function defined in an [`AST`] with multiple [`Dynamic`] arguments.
    ///
    /// Options are provided via the [`CallFnOptions`] type.
    /// This is an advanced API.
    ///
    /// Not available under `no_function`.
    ///
    /// # Example
    ///
    /// ```
    /// # fn main() -> Result<(), Box<rhai::EvalAltResult>> {
    /// use rhai::{Engine, Scope, Dynamic, CallFnOptions};
    ///
    /// let engine = Engine::new();
    ///
    /// let ast = engine.compile("
    ///     fn action(x) { this += x; }         // function using 'this' pointer
    ///     fn decl(x)   { let hello = x; }     // declaring variables
    /// ")?;
    ///
    /// let mut scope = Scope::new();
    /// scope.push("foo", 42_i64);
    ///
    /// // Binding the 'this' pointer
    /// let mut value = 1_i64.into();
    /// let options = CallFnOptions::new().bind_this_ptr(&mut value);
    ///
    /// engine.call_fn_with_options(options, &mut scope, &ast, "action", ( 41_i64, ))?;
    /// assert_eq!(value.as_int().unwrap(), 42);
    ///
    /// // Do not rewind scope
    /// let options = CallFnOptions::default().rewind_scope(false);
    ///
    /// engine.call_fn_with_options(options, &mut scope, &ast, "decl", ( 42_i64, ))?;
    /// assert_eq!(scope.get_value::<i64>("hello").unwrap(), 42);
    /// # Ok(())
    /// # }
    /// ```
    #[inline(always)]
    pub fn call_fn_with_options<T: Variant + Clone>(
        &self,
        options: CallFnOptions,
        scope: &mut Scope,
        ast: &AST,
        name: impl AsRef<str>,
        args: impl FuncArgs,
    ) -> RhaiResultOf<T> {
        let mut arg_values = StaticVec::new_const();
        args.parse(&mut arg_values);

        self._call_fn(
            scope,
            &mut GlobalRuntimeState::new(self),
            &mut Caches::new(),
            ast,
            name.as_ref(),
            arg_values.as_mut(),
            options,
        )
        .and_then(|result| {
            result.try_cast_raw().map_err(|r| {
                let result_type = self.map_type_name(r.type_name());
                let cast_type = match type_name::<T>() {
                    typ @ _ if typ.contains("::") => self.map_type_name(typ),
                    typ @ _ => typ,
                };
                ERR::ErrorMismatchOutputType(cast_type.into(), result_type.into(), Position::NONE)
                    .into()
            })
        })
    }
    /// Call a script function defined in an [`AST`] with multiple [`Dynamic`] arguments.
    ///
    /// # Arguments
    ///
    /// All the arguments are _consumed_, meaning that they're replaced by `()`. This is to avoid
    /// unnecessarily cloning the arguments.
    ///
    /// Do not use the arguments after this call. If they are needed afterwards, clone them _before_
    /// calling this function.
    #[inline(always)]
    pub(crate) fn _call_fn(
        &self,
        scope: &mut Scope,
        global: &mut GlobalRuntimeState,
        caches: &mut Caches,
        ast: &AST,
        name: &str,
        arg_values: &mut [Dynamic],
        options: CallFnOptions,
    ) -> RhaiResult {
        let statements = ast.statements();

        let orig_lib_len = global.lib.len();

        let orig_tag = options.tag.map(|v| mem::replace(&mut global.tag, v));
        let mut this_ptr = options.this_ptr;

        global.lib.push(ast.shared_lib().clone());

        #[cfg(not(feature = "no_module"))]
        let orig_embedded_module_resolver = std::mem::replace(
            &mut global.embedded_module_resolver,
            ast.resolver().cloned(),
        );

        let rewind_scope = options.rewind_scope;

        let result = if options.eval_ast && !statements.is_empty() {
            defer! {
                scope if rewind_scope => rewind;
                let orig_scope_len = scope.len();
            }

            self.eval_global_statements(global, caches, scope, statements)
        } else {
            Ok(Dynamic::UNIT)
        }
        .and_then(|_| {
            let args = &mut arg_values.iter_mut().collect::<StaticVec<_>>();

            // Check for data race.
            #[cfg(not(feature = "no_closure"))]
            crate::func::ensure_no_data_race(name, args, false)?;

            ast.shared_lib()
                .get_script_fn(name, args.len())
                .map_or_else(
                    || Err(ERR::ErrorFunctionNotFound(name.into(), Position::NONE).into()),
                    |fn_def| {
                        self.call_script_fn(
                            global,
                            caches,
                            scope,
                            this_ptr.as_deref_mut(),
                            None,
                            fn_def,
                            args,
                            rewind_scope,
                            Position::NONE,
                        )
                    },
                )
        });

        #[cfg(feature = "debugging")]
        if self.is_debugger_registered() {
            global.debugger_mut().status = crate::eval::DebuggerStatus::Terminate;
            let node = &crate::ast::Stmt::Noop(Position::NONE);
            self.run_debugger(global, caches, scope, this_ptr, node)?;
        }

        #[cfg(not(feature = "no_module"))]
        {
            global.embedded_module_resolver = orig_embedded_module_resolver;
        }

        if let Some(value) = orig_tag {
            global.tag = value;
        }

        global.lib.truncate(orig_lib_len);

        result
    }
}
