use rhai::{Dynamic, Engine, EvalAltResult, Module, ParseErrorType, Position, Scope, INT};

#[test]
fn test_var_scope() -> Result<(), Box<EvalAltResult>> {
    let engine = Engine::new();
    let mut scope = Scope::new();

    engine.run_with_scope(&mut scope, "let x = 4 + 5")?;
    assert_eq!(engine.eval_with_scope::<INT>(&mut scope, "x")?, 9);
    engine.run_with_scope(&mut scope, "x += 1; x += 2;")?;
    assert_eq!(engine.eval_with_scope::<INT>(&mut scope, "x")?, 12);

    scope.set_value("x", 42 as INT);
    assert_eq!(engine.eval_with_scope::<INT>(&mut scope, "x")?, 42);

    engine.run_with_scope(&mut scope, "{ let x = 3 }")?;
    assert_eq!(engine.eval_with_scope::<INT>(&mut scope, "x")?, 42);

    #[cfg(not(feature = "no_optimize"))]
    if engine.optimization_level() != rhai::OptimizationLevel::None {
        scope.clear();
        engine.run_with_scope(&mut scope, "let x = 3; let x = 42; let x = 123;")?;
        assert_eq!(scope.len(), 1);
        assert_eq!(scope.get_value::<INT>("x").unwrap(), 123);

        scope.clear();
        engine.run_with_scope(
            &mut scope,
            "let x = 3; let y = 0; let x = 42; let y = 999; let x = 123;",
        )?;
        assert_eq!(scope.len(), 2);
        assert_eq!(scope.get_value::<INT>("x").unwrap(), 123);
        assert_eq!(scope.get_value::<INT>("y").unwrap(), 999);

        scope.clear();
        engine.run_with_scope(
            &mut scope,
            "const x = 3; let y = 0; let x = 42; let y = 999;",
        )?;
        assert_eq!(scope.len(), 2);
        assert_eq!(scope.get_value::<INT>("x").unwrap(), 42);
        assert_eq!(scope.get_value::<INT>("y").unwrap(), 999);
        assert!(!scope.is_constant("x").unwrap());
        assert!(!scope.is_constant("y").unwrap());

        scope.clear();
        engine.run_with_scope(
            &mut scope,
            "const x = 3; let y = 0; let x = 42; let y = 999; const x = 123;",
        )?;
        assert_eq!(scope.len(), 2);
        assert_eq!(scope.get_value::<INT>("x").unwrap(), 123);
        assert_eq!(scope.get_value::<INT>("y").unwrap(), 999);
        assert!(scope.is_constant("x").unwrap());
        assert!(!scope.is_constant("y").unwrap());

        scope.clear();
        engine.run_with_scope(
            &mut scope,
            "let x = 3; let y = 0; { let x = 42; let y = 999; } let x = 123;",
        )?;

        assert_eq!(scope.len(), 2);
        assert_eq!(scope.get_value::<INT>("x").unwrap(), 123);
        assert_eq!(scope.get_value::<INT>("y").unwrap(), 0);

        assert_eq!(
            engine.eval::<INT>(
                "
                    let sum = 0;
                    for x in 0..10 {
                        let x = 42;
                        sum += x;
                    }
                    sum
                ",
            )?,
            420
        );
    }

    scope.clear();

    scope.push("x", 42 as INT);
    scope.push_constant("x", 42 as INT);

    let scope2 = scope.clone();
    let scope3 = scope.clone_visible();

    assert_eq!(scope2.is_constant("x"), Some(true));
    assert_eq!(scope3.is_constant("x"), Some(true));

    Ok(())
}

#[cfg(not(feature = "no_module"))]
#[test]
fn test_var_scope_alias() -> Result<(), Box<EvalAltResult>> {
    let engine = Engine::new();
    let mut scope = Scope::new();

    scope.push("x", 42 as INT);
    scope.set_alias("x", "a");
    scope.set_alias("x", "b");
    scope.set_alias("x", "y");
    scope.push("x", 123 as INT);
    scope.set_alias("x", "b");
    scope.set_alias("x", "c");

    let ast = engine.compile(
        "
            let x = 999;
            export x as a;
            export x as c;
            let x = 0;
            export x as z;
        ",
    )?;

    let m = Module::eval_ast_as_new(scope, &ast, &engine)?;

    assert_eq!(m.get_var_value::<INT>("a").unwrap(), 999);
    assert_eq!(m.get_var_value::<INT>("b").unwrap(), 123);
    assert_eq!(m.get_var_value::<INT>("c").unwrap(), 999);
    assert_eq!(m.get_var_value::<INT>("y").unwrap(), 42);
    assert_eq!(m.get_var_value::<INT>("z").unwrap(), 0);

    Ok(())
}

#[test]
fn test_var_is_def() -> Result<(), Box<EvalAltResult>> {
    let engine = Engine::new();

    assert!(engine.eval::<bool>(
        r#"
            let x = 42;
            is_def_var("x")
        "#
    )?);
    assert!(!engine.eval::<bool>(
        r#"
            let x = 42;
            is_def_var("y")
        "#
    )?);
    assert!(engine.eval::<bool>(
        r#"
            const x = 42;
            is_def_var("x")
        "#
    )?);

    Ok(())
}

#[test]
fn test_scope_eval() -> Result<(), Box<EvalAltResult>> {
    let engine = Engine::new();

    // First create the state
    let mut scope = Scope::new();

    // Then push some initialized variables into the state
    // NOTE: Remember the default numbers used by Rhai are INT and f64.
    //       Better stick to them or it gets hard to work with other variables in the script.
    scope.push("y", 42 as INT);
    scope.push("z", 999 as INT);

    // First invocation
    engine
        .run_with_scope(&mut scope, " let x = 4 + 5 - y + z; y = 1;")
        .expect("variables y and z should exist");

    // Second invocation using the same state
    let result = engine.eval_with_scope::<INT>(&mut scope, "x")?;

    println!("result: {result}"); // should print 966

    // Variable y is changed in the script
    assert_eq!(
        scope
            .get_value::<INT>("y")
            .expect("variable y should exist"),
        1
    );

    Ok(())
}

#[test]
fn test_var_resolver() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();

    let mut scope = Scope::new();
    scope.push("innocent", 1 as INT);
    scope.push("chameleon", 123 as INT);
    scope.push("DO_NOT_USE", 999 as INT);

    #[cfg(not(feature = "no_closure"))]
    let mut base = Dynamic::ONE.into_shared();
    #[cfg(not(feature = "no_closure"))]
    let shared = base.clone();

    #[allow(deprecated)] // not deprecated but unstable
    engine.on_var(move |name, _, context| {
        match name {
            "MYSTIC_NUMBER" => Ok(Some((42 as INT).into())),
            #[cfg(not(feature = "no_closure"))]
            "HELLO" => Ok(Some(shared.clone())),
            // Override a variable - make it not found even if it exists!
            "DO_NOT_USE" => {
                Err(EvalAltResult::ErrorVariableNotFound(name.to_string(), Position::NONE).into())
            }
            // Silently maps 'chameleon' into 'innocent'.
            "chameleon" => context
                .scope()
                .get_value("innocent")
                .map(Some)
                .ok_or_else(|| {
                    EvalAltResult::ErrorVariableNotFound(name.to_string(), Position::NONE).into()
                }),
            // Return Ok(None) to continue with the normal variable resolution process.
            _ => Ok(None),
        }
    });

    assert_eq!(
        engine.eval_with_scope::<INT>(&mut scope, "MYSTIC_NUMBER")?,
        42
    );

    #[cfg(not(feature = "no_closure"))]
    {
        assert_eq!(engine.eval::<INT>("HELLO")?, 1);
        *base.write_lock::<INT>().unwrap() = 42;
        assert_eq!(engine.eval::<INT>("HELLO")?, 42);
        engine.run("HELLO = 123")?;
        assert_eq!(base.as_int().unwrap(), 123);
        assert_eq!(engine.eval::<INT>("HELLO = HELLO + 1; HELLO")?, 124);
        assert_eq!(engine.eval::<INT>("HELLO = HELLO * 2; HELLO")?, 248);
        assert_eq!(base.as_int().unwrap(), 248);
    }

    assert_eq!(engine.eval_with_scope::<INT>(&mut scope, "chameleon")?, 1);
    assert!(
        matches!(*engine.eval_with_scope::<INT>(&mut scope, "DO_NOT_USE").unwrap_err(),
        EvalAltResult::ErrorVariableNotFound(n, ..) if n == "DO_NOT_USE")
    );

    Ok(())
}

#[test]
fn test_var_def_filter() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();

    let ast = engine.compile("let x = 42;")?;
    engine.run_ast(&ast)?;

    #[allow(deprecated)] // not deprecated but unstable
    engine.on_def_var(|_, info, _| match (info.name, info.nesting_level) {
        ("x", 0 | 1) => Ok(false),
        _ => Ok(true),
    });

    assert_eq!(
        engine.eval::<INT>("let y = 42; let y = 123; let z = y + 1; z")?,
        124
    );

    assert!(matches!(
        engine.compile("let x = 42;").unwrap_err().err_type(),
        ParseErrorType::ForbiddenVariable(s) if s == "x"
    ));
    assert!(matches!(
        *engine.run_ast(&ast).expect_err("should err"),
        EvalAltResult::ErrorForbiddenVariable(s, _) if s == "x"
    ));
    assert!(engine.run("const x = 42;").is_err());
    assert!(engine.run("let y = 42; { let x = y + 1; }").is_err());
    assert!(engine.run("let y = 42; { let x = y + 1; }").is_err());
    engine.run("let y = 42; { let z = y + 1; { let x = z + 1; } }")?;

    Ok(())
}
