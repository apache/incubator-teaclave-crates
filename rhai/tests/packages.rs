use rhai::packages::{Package, StandardPackage as SSS};
use rhai::{def_package, Engine, EvalAltResult, Module, Scope, INT};

#[cfg(not(feature = "no_module"))]
#[cfg(not(feature = "no_custom_syntax"))]
#[test]
fn test_packages() -> Result<(), Box<EvalAltResult>> {
    def_package! {
        /// My custom package.
        MyPackage(m) : SSS {
            m.set_native_fn("hello", |x: INT| Ok(x + 1));
            m.set_native_fn("@", |x: INT, y: INT| Ok(x * x + y * y));
        } |> |engine| {
            engine.register_custom_operator("@", 160).unwrap();
        }
    }

    let pkg = MyPackage::new();

    let make_call = |x: INT| -> Result<INT, Box<EvalAltResult>> {
        // Create a raw Engine - extremely cheap.
        let mut engine = Engine::new_raw();

        // Register packages - cheap.
        pkg.register_into_engine(&mut engine);
        pkg.register_into_engine_as(&mut engine, "foo");

        // Create custom scope - cheap.
        let mut scope = Scope::new();

        // Push variable into scope - relatively cheap.
        scope.push("x", x);

        // Evaluate script.

        engine.eval_with_scope::<INT>(&mut scope, "hello(x) @ foo::hello(x)")
    };

    assert_eq!(make_call(42)?, 3698);

    Ok(())
}

#[cfg(not(feature = "no_function"))]
#[cfg(not(feature = "no_module"))]
#[test]
fn test_packages_with_script() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();
    let ast = engine.compile("fn foo(x) { x + 1 }  fn bar(x) { foo(x) + 1 }")?;

    let module = Module::eval_ast_as_new(Scope::new(), &ast, &engine)?;
    engine.register_global_module(module.into());
    assert_eq!(engine.eval::<INT>("foo(41)")?, 42);
    assert_eq!(engine.eval::<INT>("bar(40)")?, 42);

    Ok(())
}
