//! The `FnPtr` type.

use crate::eval::GlobalRuntimeState;
use crate::func::EncapsulatedEnviron;
use crate::tokenizer::{is_reserved_keyword_or_symbol, is_valid_function_name, Token};
use crate::types::dynamic::Variant;
use crate::{
    Dynamic, Engine, FnArgsVec, FuncArgs, ImmutableString, NativeCallContext, ParseErrorType,
    Position, RhaiError, RhaiResult, RhaiResultOf, Shared, StaticVec, AST, ERR,
};
#[cfg(feature = "no_std")]
use std::prelude::v1::*;
use std::{
    any::type_name,
    convert::{TryFrom, TryInto},
    fmt,
    hash::{Hash, Hasher},
    mem,
    ops::{Index, IndexMut},
};

/// A general function pointer, which may carry additional (i.e. curried) argument values
/// to be passed onto a function during a call.
#[derive(Clone)]
pub struct FnPtr {
    name: ImmutableString,
    curry: StaticVec<Dynamic>,
    environ: Option<Shared<EncapsulatedEnviron>>,
    #[cfg(not(feature = "no_function"))]
    fn_def: Option<Shared<crate::ast::ScriptFnDef>>,
}

impl Hash for FnPtr {
    #[inline(always)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.curry.hash(state);

        // Hash the shared [`EncapsulatedEnviron`] by hashing its shared pointer.
        self.environ.as_ref().map(Shared::as_ptr).hash(state);

        // Hash the linked [`ScriptFnDef`][crate::ast::ScriptFnDef] by hashing its shared pointer.
        #[cfg(not(feature = "no_function"))]
        self.fn_def.as_ref().map(Shared::as_ptr).hash(state);
    }
}

impl fmt::Debug for FnPtr {
    #[cold]
    #[inline(never)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ff = &mut f.debug_tuple("Fn");
        ff.field(&self.name);
        self.curry.iter().for_each(|curry| {
            ff.field(curry);
        });
        ff.finish()?;

        #[cfg(not(feature = "no_function"))]
        if let Some(ref fn_def) = self.fn_def {
            write!(f, ": {fn_def}")?;
        }

        Ok(())
    }
}

impl FnPtr {
    /// Create a new function pointer.
    #[inline(always)]
    pub fn new(name: impl Into<ImmutableString>) -> RhaiResultOf<Self> {
        name.into().try_into()
    }
    /// Create a new function pointer without checking its parameters.
    #[inline(always)]
    #[must_use]
    #[allow(dead_code)]
    pub(crate) fn new_unchecked(
        name: impl Into<ImmutableString>,
        curry: StaticVec<Dynamic>,
    ) -> Self {
        Self {
            name: name.into(),
            curry,
            environ: None,
            #[cfg(not(feature = "no_function"))]
            fn_def: None,
        }
    }
    /// Get the name of the function.
    #[inline(always)]
    #[must_use]
    pub fn fn_name(&self) -> &str {
        self.fn_name_raw().as_str()
    }
    /// Get the name of the function.
    #[inline(always)]
    #[must_use]
    pub(crate) const fn fn_name_raw(&self) -> &ImmutableString {
        &self.name
    }
    /// Get the underlying data of the function pointer.
    #[cfg(not(feature = "no_function"))]
    #[inline(always)]
    #[must_use]
    pub(crate) fn take_data(
        self,
    ) -> (
        ImmutableString,
        StaticVec<Dynamic>,
        Option<Shared<EncapsulatedEnviron>>,
        Option<Shared<crate::ast::ScriptFnDef>>,
    ) {
        (self.name, self.curry, self.environ, self.fn_def)
    }
    /// Get the underlying data of the function pointer.
    #[cfg(feature = "no_function")]
    #[inline(always)]
    #[must_use]
    pub(crate) fn take_data(
        self,
    ) -> (
        ImmutableString,
        StaticVec<Dynamic>,
        Option<Shared<EncapsulatedEnviron>>,
    ) {
        (self.name, self.curry, self.environ)
    }
    /// Get the curried arguments.
    #[inline(always)]
    pub fn curry(&self) -> &[Dynamic] {
        self.curry.as_ref()
    }
    /// Iterate the curried arguments.
    #[inline(always)]
    pub fn iter_curry(&self) -> impl Iterator<Item = &Dynamic> {
        self.curry.iter()
    }
    /// Mutably-iterate the curried arguments.
    #[inline(always)]
    pub fn iter_curry_mut(&mut self) -> impl Iterator<Item = &mut Dynamic> {
        self.curry.iter_mut()
    }
    /// Add a new curried argument.
    #[inline(always)]
    pub fn add_curry(&mut self, value: Dynamic) -> &mut Self {
        self.curry.push(value);
        self
    }
    /// Set curried arguments to the function pointer.
    #[inline]
    pub fn set_curry(&mut self, values: impl IntoIterator<Item = Dynamic>) -> &mut Self {
        self.curry = values.into_iter().collect();
        self
    }
    /// Is the function pointer curried?
    #[inline(always)]
    #[must_use]
    pub fn is_curried(&self) -> bool {
        !self.curry.is_empty()
    }
    /// Does the function pointer refer to an anonymous function?
    ///
    /// Not available under `no_function`.
    #[cfg(not(feature = "no_function"))]
    #[inline(always)]
    #[must_use]
    pub fn is_anonymous(&self) -> bool {
        crate::func::is_anonymous_fn(&self.name)
    }
    /// Call the function pointer with curried arguments (if any).
    /// The function may be script-defined (not available under `no_function`) or native Rust.
    ///
    /// This method is intended for calling a function pointer directly, possibly on another [`Engine`].
    /// Therefore, the [`AST`] is _NOT_ evaluated before calling the function.
    ///
    /// # Example
    ///
    /// ```
    /// # fn main() -> Result<(), Box<rhai::EvalAltResult>> {
    /// # #[cfg(not(feature = "no_function"))]
    /// # {
    /// use rhai::{Engine, FnPtr};
    ///
    /// let engine = Engine::new();
    ///
    /// let ast = engine.compile("fn foo(x, y) { len(x) + y }")?;
    ///
    /// let mut fn_ptr = FnPtr::new("foo")?;
    ///
    /// // Curry values into the function pointer
    /// fn_ptr.set_curry(vec!["abc".into()]);
    ///
    /// // Values are only needed for non-curried parameters
    /// let result: i64 = fn_ptr.call(&engine, &ast, ( 39_i64, ) )?;
    ///
    /// assert_eq!(result, 42);
    /// # }
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn call<T: Variant + Clone>(
        &self,
        engine: &Engine,
        ast: &AST,
        args: impl FuncArgs,
    ) -> RhaiResultOf<T> {
        let _ast = ast;
        let mut arg_values = StaticVec::new_const();
        args.parse(&mut arg_values);

        let global = &mut GlobalRuntimeState::new(engine);

        #[cfg(not(feature = "no_function"))]
        global.lib.push(_ast.shared_lib().clone());

        let ctx = (engine, self.fn_name(), None, &*global, Position::NONE).into();

        self.call_raw(&ctx, None, arg_values).and_then(|result| {
            result.try_cast_raw().map_err(|r| {
                let result_type = engine.map_type_name(r.type_name());
                let cast_type = match type_name::<T>() {
                    typ @ _ if typ.contains("::") => engine.map_type_name(typ),
                    typ @ _ => typ,
                };
                ERR::ErrorMismatchOutputType(cast_type.into(), result_type.into(), Position::NONE)
                    .into()
            })
        })
    }
    /// Call the function pointer with curried arguments (if any).
    /// The function may be script-defined (not available under `no_function`) or native Rust.
    ///
    /// This method is intended for calling a function pointer that is passed into a native Rust
    /// function as an argument.  Therefore, the [`AST`] is _NOT_ evaluated before calling the
    /// function.
    #[inline]
    pub fn call_within_context<T: Variant + Clone>(
        &self,
        context: &NativeCallContext,
        args: impl FuncArgs,
    ) -> RhaiResultOf<T> {
        let mut arg_values = StaticVec::new_const();
        args.parse(&mut arg_values);

        self.call_raw(context, None, arg_values).and_then(|result| {
            result.try_cast_raw().map_err(|r| {
                let result_type = context.engine().map_type_name(r.type_name());
                let cast_type = match type_name::<T>() {
                    typ @ _ if typ.contains("::") => context.engine().map_type_name(typ),
                    typ @ _ => typ,
                };
                ERR::ErrorMismatchOutputType(cast_type.into(), result_type.into(), Position::NONE)
                    .into()
            })
        })
    }
    /// Call the function pointer with curried arguments (if any).
    /// The function may be script-defined (not available under `no_function`) or native Rust.
    ///
    /// This method is intended for calling a function pointer that is passed into a native Rust
    /// function as an argument.  Therefore, the [`AST`] is _NOT_ evaluated before calling the
    /// function.
    ///
    /// # WARNING - Low Level API
    ///
    /// This function is very low level.
    ///
    /// # Arguments
    ///
    /// All the arguments are _consumed_, meaning that they're replaced by `()`.
    /// This is to avoid unnecessarily cloning the arguments.
    ///
    /// Do not use the arguments after this call. If they are needed afterwards,
    /// clone them _before_ calling this function.
    #[inline]
    pub fn call_raw(
        &self,
        context: &NativeCallContext,
        this_ptr: Option<&mut Dynamic>,
        arg_values: impl AsMut<[Dynamic]>,
    ) -> RhaiResult {
        let mut arg_values = arg_values;
        let mut arg_values = arg_values.as_mut();
        let mut args_data;

        if self.is_curried() {
            args_data = FnArgsVec::with_capacity(self.curry().len() + arg_values.len());
            args_data.extend(self.curry().iter().cloned());
            args_data.extend(arg_values.iter_mut().map(mem::take));
            arg_values = &mut *args_data;
        };

        let args = &mut StaticVec::with_capacity(arg_values.len() + 1);
        args.extend(arg_values.iter_mut());

        // Linked to scripted function?
        #[cfg(not(feature = "no_function"))]
        if let Some(ref fn_def) = self.fn_def {
            if fn_def.params.len() == args.len() {
                let global = &mut context.global_runtime_state().clone();
                global.level += 1;

                let caches = &mut crate::eval::Caches::new();

                return context.engine().call_script_fn(
                    global,
                    caches,
                    &mut crate::Scope::new(),
                    this_ptr,
                    self.encapsulated_environ().map(|r| r.as_ref()),
                    fn_def,
                    args,
                    true,
                    context.position(),
                );
            }
        }

        let is_method = this_ptr.is_some();

        if let Some(obj) = this_ptr {
            args.insert(0, obj);
        }

        context.call_fn_raw(self.fn_name(), is_method, is_method, args)
    }
    /// Get a reference to the [encapsulated environment][EncapsulatedEnviron].
    #[inline(always)]
    #[must_use]
    #[allow(dead_code)]
    pub(crate) fn encapsulated_environ(&self) -> Option<&Shared<EncapsulatedEnviron>> {
        self.environ.as_ref()
    }
    /// Set a reference to the [encapsulated environment][EncapsulatedEnviron].
    #[inline(always)]
    #[allow(dead_code)]
    pub(crate) fn set_encapsulated_environ(
        &mut self,
        value: Option<impl Into<Shared<EncapsulatedEnviron>>>,
    ) {
        self.environ = value.map(Into::into);
    }
    /// Get a reference to the linked [`ScriptFnDef`][crate::ast::ScriptFnDef].
    #[cfg(not(feature = "no_function"))]
    #[inline(always)]
    #[must_use]
    pub(crate) fn fn_def(&self) -> Option<&Shared<crate::ast::ScriptFnDef>> {
        self.fn_def.as_ref()
    }
    /// Set a reference to the linked [`ScriptFnDef`][crate::ast::ScriptFnDef].
    #[cfg(not(feature = "no_function"))]
    #[inline(always)]
    pub(crate) fn set_fn_def(&mut self, value: Option<impl Into<Shared<crate::ast::ScriptFnDef>>>) {
        self.fn_def = value.map(Into::into);
    }

    /// Make a call to a function pointer with either a specified number of arguments, or with extra
    /// arguments attached.
    ///
    /// If `this_ptr` is provided, it is first provided to script-defined functions bound to `this`.
    ///
    /// When an appropriate function is not found and `move_this_ptr_to_args` is `Some`, `this_ptr`
    /// is removed and inserted as the appropriate parameter number.
    ///
    /// This is useful for calling predicate closures within an iteration loop where the extra argument
    /// is the current element's index.
    ///
    /// If the function pointer is linked to a scripted function definition, use the appropriate number
    /// of arguments to call it directly (one version attaches extra arguments).
    #[cfg(not(feature = "internals"))]
    #[inline(always)]
    #[allow(dead_code)]
    pub(crate) fn call_raw_with_extra_args<const N: usize, const E: usize>(
        &self,
        fn_name: &str,
        ctx: &NativeCallContext,
        this_ptr: Option<&mut Dynamic>,
        args: [Dynamic; N],
        extras: [Dynamic; E],
        move_this_ptr_to_args: Option<usize>,
    ) -> RhaiResult {
        self._call_with_extra_args(fn_name, ctx, this_ptr, args, extras, move_this_ptr_to_args)
    }
    /// _(internals)_ Make a call to a function pointer with either a specified number of arguments,
    /// or with extra arguments attached. Exported under the `internals` feature only.
    ///
    /// If `this_ptr` is provided, it is first provided to script-defined functions bound to `this`.
    ///
    /// When an appropriate function is not found and `move_this_ptr_to_args` is `Some`, `this_ptr`
    /// is removed and inserted as the appropriate parameter number.
    ///
    /// This is useful for calling predicate closures within an iteration loop where the extra
    /// argument is the current element's index.
    ///
    /// If the function pointer is linked to a scripted function definition, use the appropriate
    /// number of arguments to call it directly (one version attaches extra arguments).
    #[cfg(feature = "internals")]
    #[inline(always)]
    pub fn call_raw_with_extra_args<const N: usize, const E: usize>(
        &self,
        fn_name: &str,
        ctx: &NativeCallContext,
        this_ptr: Option<&mut Dynamic>,
        args: [Dynamic; N],
        extras: [Dynamic; E],
        move_this_ptr_to_args: Option<usize>,
    ) -> RhaiResult {
        self._call_with_extra_args(fn_name, ctx, this_ptr, args, extras, move_this_ptr_to_args)
    }
    /// Make a call to a function pointer with either a specified number of arguments, or with extra
    /// arguments attached.
    fn _call_with_extra_args<const N: usize, const E: usize>(
        &self,
        fn_name: &str,
        ctx: &NativeCallContext,
        mut this_ptr: Option<&mut Dynamic>,
        args: [Dynamic; N],
        extras: [Dynamic; E],
        move_this_ptr_to_args: Option<usize>,
    ) -> RhaiResult {
        #[cfg(not(feature = "no_function"))]
        if let Some(arity) = self.fn_def().map(|f| f.params.len()) {
            if arity == N + self.curry().len() {
                return self.call_raw(ctx, this_ptr, args);
            }
            if let Some(move_to_args) = move_this_ptr_to_args {
                if this_ptr.is_some() {
                    if arity == N + 1 + self.curry().len() {
                        let mut args2 = FnArgsVec::with_capacity(args.len() + 1);
                        if move_to_args == 0 {
                            args2.push(this_ptr.as_mut().unwrap().clone());
                            args2.extend(args);
                        } else {
                            args2.extend(args);
                            args2.insert(move_to_args, this_ptr.as_mut().unwrap().clone());
                        }
                        return self.call_raw(ctx, None, args2);
                    }
                    if arity == N + E + 1 + self.curry().len() {
                        let mut args2 = FnArgsVec::with_capacity(args.len() + extras.len() + 1);
                        if move_to_args == 0 {
                            args2.push(this_ptr.as_mut().unwrap().clone());
                            args2.extend(args);
                            args2.extend(extras);
                        } else {
                            args2.extend(args);
                            args2.insert(move_to_args, this_ptr.as_mut().unwrap().clone());
                            args2.extend(extras);
                        }
                        return self.call_raw(ctx, None, args2);
                    }
                }
            }
            if arity == N + E + self.curry().len() {
                let mut args2 = FnArgsVec::with_capacity(args.len() + extras.len());
                args2.extend(args);
                args2.extend(extras);
                return self.call_raw(ctx, this_ptr, args2);
            }
        }

        self.call_raw(ctx, this_ptr.as_deref_mut(), args.clone())
            .or_else(|err| match *err {
                ERR::ErrorFunctionNotFound(sig, ..)
                    if move_this_ptr_to_args.is_some()
                        && this_ptr.is_some()
                        && sig.starts_with(self.fn_name()) =>
                {
                    let mut args2 = FnArgsVec::with_capacity(args.len() + 1);
                    let move_to_args = move_this_ptr_to_args.unwrap();
                    if move_to_args == 0 {
                        args2.push(this_ptr.as_mut().unwrap().clone());
                        args2.extend(args.clone());
                    } else {
                        args2.extend(args.clone());
                        args2.insert(move_to_args, this_ptr.as_mut().unwrap().clone());
                    }
                    self.call_raw(ctx, None, args2)
                }
                _ => Err(err),
            })
            .or_else(|err| match *err {
                ERR::ErrorFunctionNotFound(sig, ..) if sig.starts_with(self.fn_name()) => {
                    if let Some(move_to_args) = move_this_ptr_to_args {
                        if let Some(ref mut this_ptr) = this_ptr {
                            let mut args2 = FnArgsVec::with_capacity(args.len() + extras.len() + 1);
                            if move_to_args == 0 {
                                args2.push(this_ptr.clone());
                                args2.extend(args);
                                args2.extend(extras);
                            } else {
                                args2.extend(args);
                                args2.extend(extras);
                                args2.insert(move_to_args, this_ptr.clone());
                            }
                            return self.call_raw(ctx, None, args2);
                        }
                    }

                    let mut args2 = FnArgsVec::with_capacity(args.len() + extras.len());
                    args2.extend(args);
                    args2.extend(extras);

                    self.call_raw(ctx, this_ptr, args2)
                }
                _ => Err(err),
            })
            .map_err(|err| {
                Box::new(ERR::ErrorInFunctionCall(
                    fn_name.to_string(),
                    ctx.source().unwrap_or("").to_string(),
                    err,
                    Position::NONE,
                ))
            })
    }
}

impl fmt::Display for FnPtr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Fn({})", self.fn_name())
    }
}

impl TryFrom<ImmutableString> for FnPtr {
    type Error = RhaiError;

    #[inline(always)]
    fn try_from(value: ImmutableString) -> RhaiResultOf<Self> {
        if is_valid_function_name(&value) {
            Ok(Self {
                name: value,
                curry: StaticVec::new_const(),
                environ: None,
                #[cfg(not(feature = "no_function"))]
                fn_def: None,
            })
        } else if is_reserved_keyword_or_symbol(&value).0
            || Token::lookup_symbol_from_syntax(&value).is_some()
        {
            Err(
                ERR::ErrorParsing(ParseErrorType::Reserved(value.to_string()), Position::NONE)
                    .into(),
            )
        } else {
            Err(ERR::ErrorFunctionNotFound(value.to_string(), Position::NONE).into())
        }
    }
}

#[cfg(not(feature = "no_function"))]
impl<T: Into<Shared<crate::ast::ScriptFnDef>>> From<T> for FnPtr {
    #[inline(always)]
    fn from(value: T) -> Self {
        let fn_def = value.into();

        Self {
            name: fn_def.name.clone(),
            curry: StaticVec::new_const(),
            environ: None,
            fn_def: Some(fn_def),
        }
    }
}

impl Index<usize> for FnPtr {
    type Output = Dynamic;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        self.curry.index(index)
    }
}

impl IndexMut<usize> for FnPtr {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.curry.index_mut(index)
    }
}

impl Extend<Dynamic> for FnPtr {
    #[inline(always)]
    fn extend<T: IntoIterator<Item = Dynamic>>(&mut self, iter: T) {
        self.curry.extend(iter)
    }
}
