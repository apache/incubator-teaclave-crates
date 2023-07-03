#![cfg(not(feature = "no_object"))]

use rhai::{Engine, EvalAltResult, Map, ParseErrorType, Scope, INT};

#[test]
fn test_map_indexing() -> Result<(), Box<EvalAltResult>> {
    let engine = Engine::new();

    #[cfg(not(feature = "no_index"))]
    {
        assert_eq!(
            engine.eval::<INT>(r#"let x = #{a: 1, b: 2, c: 3}; x["b"]"#)?,
            2
        );
        assert_eq!(
            engine.eval::<INT>(r#"let x = #{a: 1, b: 2, c: 3,}; x["b"]"#)?,
            2
        );
        assert_eq!(
            engine.eval::<char>(
                r#"
                    let y = #{d: 1, "e": #{a: 42, b: 88, "": "hello"}, " 123 xyz": 9};
                    y.e[""][4]
                "#
            )?,
            'o'
        );
        assert_eq!(
            engine.eval::<String>(r#"let a = [#{s:"hello"}]; a[0].s[2] = 'X'; a[0].s"#)?,
            "heXlo"
        );
    }

    assert_eq!(
        engine.eval::<INT>("let y = #{a: 1, b: 2, c: 3}; y.a = 5; y.a")?,
        5
    );

    engine.run("let y = #{a: 1, b: 2, c: 3}; y.z")?;

    #[cfg(not(feature = "no_index"))]
    assert_eq!(
        engine.eval::<INT>(
            r#"
                let y = #{`a
b`: 1}; y["a\nb"]
            "#
        )?,
        1
    );

    assert!(matches!(
        *engine
            .eval::<INT>("let y = #{`a${1}`: 1}; y.a1")
            .unwrap_err(),
        EvalAltResult::ErrorParsing(ParseErrorType::PropertyExpected, ..)
    ));

    assert!(engine.eval::<bool>(r#"let y = #{a: 1, b: 2, c: 3}; "c" in y"#)?);
    assert!(engine.eval::<bool>(r#"let y = #{a: 1, b: 2, c: 3}; "b" in y"#)?);
    assert!(!engine.eval::<bool>(r#"let y = #{a: 1, b: 2, c: 3}; "z" in y"#)?);

    assert_eq!(
        engine.eval::<INT>(
            r#"
                let x = #{a: 1, b: 2, c: 3};
                let c = x.remove("c");
                x.len() + c
            "#
        )?,
        5
    );
    assert_eq!(
        engine.eval::<INT>(
            "
                let x = #{a: 1, b: 2, c: 3};
                let y = #{b: 42, d: 9};
                x.mixin(y);
                x.len() + x.b
            "
        )?,
        46
    );
    assert_eq!(
        engine.eval::<INT>(
            "
                let x = #{a: 1, b: 2, c: 3};
                x += #{b: 42, d: 9};
                x.len() + x.b
            "
        )?,
        46
    );
    assert_eq!(
        engine
            .eval::<Map>(
                "
                    let x = #{a: 1, b: 2, c: 3};
                    let y = #{b: 42, d: 9};
                    x + y
                "
            )?
            .len(),
        4
    );

    Ok(())
}

#[test]
fn test_map_prop() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();

    engine.eval::<()>("let x = #{a: 42}; x.b").unwrap();

    engine.set_fail_on_invalid_map_property(true);

    assert!(
        matches!(*engine.eval::<()>("let x = #{a: 42}; x.b").unwrap_err(),
            EvalAltResult::ErrorPropertyNotFound(prop, _) if prop == "b"
        )
    );

    Ok(())
}

#[cfg(not(feature = "no_index"))]
#[test]
fn test_map_index_types() -> Result<(), Box<EvalAltResult>> {
    let engine = Engine::new();

    engine.compile(r#"#{a:1, b:2, c:3}["a"]['x']"#)?;

    assert!(matches!(
        engine
            .compile("#{a:1, b:2, c:3}['x']")
            .unwrap_err()
            .err_type(),
        ParseErrorType::MalformedIndexExpr(..)
    ));

    assert!(matches!(
        engine
            .compile("#{a:1, b:2, c:3}[1]")
            .unwrap_err()
            .err_type(),
        ParseErrorType::MalformedIndexExpr(..)
    ));

    #[cfg(not(feature = "no_float"))]
    assert!(matches!(
        engine
            .compile("#{a:1, b:2, c:3}[123.456]")
            .unwrap_err()
            .err_type(),
        ParseErrorType::MalformedIndexExpr(..)
    ));

    assert!(matches!(
        engine
            .compile("#{a:1, b:2, c:3}[()]")
            .unwrap_err()
            .err_type(),
        ParseErrorType::MalformedIndexExpr(..)
    ));

    assert!(matches!(
        engine
            .compile("#{a:1, b:2, c:3}[true && false]")
            .unwrap_err()
            .err_type(),
        ParseErrorType::MalformedIndexExpr(..)
    ));

    Ok(())
}

#[test]
fn test_map_assign() -> Result<(), Box<EvalAltResult>> {
    let engine = Engine::new();

    let x = engine.eval::<Map>(r#"let x = #{a: 1, b: true, "c$": "hello"}; x"#)?;

    assert_eq!(x["a"].as_int().unwrap(), 1);
    assert!(x["b"].as_bool().unwrap());
    assert_eq!(x["c$"].clone_cast::<String>(), "hello");

    Ok(())
}

#[test]
fn test_map_return() -> Result<(), Box<EvalAltResult>> {
    let engine = Engine::new();

    let x = engine.eval::<Map>(r#"#{a: 1, b: true, "c$": "hello"}"#)?;

    assert_eq!(x["a"].as_int().unwrap(), 1);
    assert!(x["b"].as_bool().unwrap());
    assert_eq!(x["c$"].clone_cast::<String>(), "hello");

    Ok(())
}

#[test]
#[cfg(not(feature = "no_index"))]
fn test_map_for() -> Result<(), Box<EvalAltResult>> {
    let engine = Engine::new();

    assert_eq!(
        engine
            .eval::<String>(
                r#"
                    let map = #{a: 1, b_x: true, "$c d e!": "hello"};
                    let s = "";

                    for key in keys(map) {
                        s += key;
                    }

                    s
                "#
            )?
            .len(),
        11
    );

    Ok(())
}

#[test]
/// Because a Rhai object map literal is almost the same as JSON,
/// it is possible to convert from JSON into a Rhai object map.
fn test_map_json() -> Result<(), Box<EvalAltResult>> {
    let engine = Engine::new();

    let json = r#"{"a":1, "b":true, "c":41+1, "$d e f!":"hello", "z":null}"#;

    let map = engine.parse_json(json, true)?;

    assert!(!map.contains_key("x"));

    assert_eq!(map["a"].as_int().unwrap(), 1);
    assert!(map["b"].as_bool().unwrap());
    assert_eq!(map["c"].as_int().unwrap(), 42);
    assert_eq!(map["$d e f!"].clone_cast::<String>(), "hello");
    let _: () = map["z"].as_unit().unwrap();

    #[cfg(not(feature = "no_index"))]
    {
        let mut scope = Scope::new();
        scope.push_constant("map", map);

        assert_eq!(
            engine
                .eval_with_scope::<String>(
                    &mut scope,
                    r#"
                        let s = "";

                        for key in keys(map) {
                            s += key;
                        }

                        s
                    "#
                )?
                .len(),
            11
        );
    }

    engine.parse_json(json, true)?;

    assert!(matches!(
        *engine.parse_json("123", true).unwrap_err(),
        EvalAltResult::ErrorMismatchOutputType(..)
    ));

    assert!(matches!(
        *engine.parse_json("{a:42}", true).unwrap_err(),
        EvalAltResult::ErrorParsing(..)
    ));

    assert!(matches!(
        *engine.parse_json("#{a:123}", true).unwrap_err(),
        EvalAltResult::ErrorParsing(..)
    ));

    assert!(matches!(
        *engine.parse_json("{a:()}", true).unwrap_err(),
        EvalAltResult::ErrorParsing(..)
    ));

    assert!(matches!(
        *engine.parse_json("#{a:123+456}", true).unwrap_err(),
        EvalAltResult::ErrorParsing(..)
    ));

    assert!(matches!(
        *engine.parse_json("{a:`hello${world}`}", true).unwrap_err(),
        EvalAltResult::ErrorParsing(..)
    ));

    Ok(())
}

#[test]
#[cfg(not(feature = "no_function"))]
fn test_map_oop() -> Result<(), Box<EvalAltResult>> {
    let engine = Engine::new();

    assert_eq!(
        engine.eval::<INT>(
            r#"
                let obj = #{ data: 40, action: Fn("abc") };

                fn abc(x) { this.data += x; }

                obj.action(2);
                obj.data
            "#,
        )?,
        42
    );

    Ok(())
}
