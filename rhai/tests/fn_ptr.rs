use rhai::{Engine, EvalAltResult, FnPtr, INT};

#[test]
fn test_fn_ptr() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();

    engine.register_fn("bar", |x: &mut INT, y: INT| *x += y);

    #[cfg(not(feature = "no_object"))]
    assert_eq!(
        engine.eval::<INT>(
            r#"
                let f = Fn("bar");
                let x = 40;
                f.call(x, 2);
                x
            "#
        )?,
        40
    );

    #[cfg(not(feature = "no_object"))]
    assert_eq!(
        engine.eval::<INT>(
            r#"
                let f = Fn("bar");
                let x = 40;
                x.call(f, 2);
                x
            "#
        )?,
        42
    );

    assert_eq!(
        engine.eval::<INT>(
            r#"
                let f = Fn("bar");
                let x = 40;
                call(f, x, 2);
                x
            "#
        )?,
        42
    );

    #[cfg(not(feature = "no_function"))]
    #[cfg(not(feature = "no_object"))]
    assert_eq!(
        engine.eval::<INT>(
            r#"
                fn foo(x) { this += x; }

                let f = Fn("foo");
                let x = 40;
                x.call(f, 2);
                x
            "#
        )?,
        42
    );

    #[cfg(not(feature = "no_function"))]
    assert!(matches!(
        *engine
            .eval::<INT>(
                r#"
                    fn foo(x) { this += x; }

                    let f = Fn("foo");
                    call(f, 2);
                    x
                "#
            )
            .unwrap_err(),
        EvalAltResult::ErrorInFunctionCall(fn_name, _, err, ..)
            if fn_name == "foo" && matches!(*err, EvalAltResult::ErrorUnboundThis(..))
    ));

    #[cfg(not(feature = "no_function"))]
    assert_eq!(
        engine.eval::<INT>(
            r#"
                fn foo(x) { x + 1 }
                let f = foo;
                let g = 42;
                g = foo;
                call(f, 39) + call(g, 1)
            "#
        )?,
        42
    );

    Ok(())
}

#[test]
fn test_fn_ptr_curry() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();

    engine.register_fn("foo", |x: &mut INT, y: INT| *x + y);

    #[cfg(not(feature = "no_object"))]
    assert_eq!(
        engine.eval::<INT>(
            r#"
                let f = Fn("foo");
                let f2 = f.curry(40);
                f2.call(2)
            "#
        )?,
        42
    );

    assert_eq!(
        engine.eval::<INT>(
            r#"
                let f = Fn("foo");
                let f2 = curry(f, 40);
                call(f2, 2)
            "#
        )?,
        42
    );

    Ok(())
}

#[test]
#[cfg(not(feature = "no_function"))]
fn test_fn_ptr_call() -> Result<(), Box<EvalAltResult>> {
    let engine = Engine::new();

    let ast = engine.compile("private fn foo(x, y) { len(x) + y }")?;

    let mut fn_ptr = FnPtr::new("foo")?;
    fn_ptr.set_curry(vec!["abc".into()]);
    let result: INT = fn_ptr.call(&engine, &ast, (39 as INT,))?;

    assert_eq!(result, 42);

    Ok(())
}

#[test]
#[cfg(not(feature = "no_closure"))]
fn test_fn_ptr_make_closure() -> Result<(), Box<EvalAltResult>> {
    let f = {
        let engine = Engine::new();

        let ast = engine.compile(
            r#"
                let test = "hello";
                |x| test + x            // this creates a closure
            "#,
        )?;

        let fn_ptr = engine.eval_ast::<FnPtr>(&ast)?;

        move |x: INT| -> Result<String, _> { fn_ptr.call(&engine, &ast, (x,)) }
    };

    // 'f' captures: the Engine, the AST, and the closure
    assert_eq!(f(42)?, "hello42");

    Ok(())
}
