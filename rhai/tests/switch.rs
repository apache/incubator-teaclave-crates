use rhai::{Engine, EvalAltResult, ParseErrorType, Scope, INT};

#[test]
fn test_switch() -> Result<(), Box<EvalAltResult>> {
    let engine = Engine::new();
    let mut scope = Scope::new();
    scope.push("x", 42 as INT);

    assert_eq!(
        engine.eval::<char>("switch 2 { 1 => (), 2 => 'a', 42 => true }")?,
        'a'
    );
    engine.run("switch 3 { 1 => (), 2 => 'a', 42 => true }")?;
    assert_eq!(
        engine.eval::<INT>("switch 3 { 1 => (), 2 => 'a', 42 => true, _ => 123 }")?,
        123
    );
    assert_eq!(
        engine.eval_with_scope::<INT>(
            &mut scope,
            "switch 2 { 1 => (), 2 if x < 40 => 'a', 42 => true, _ => 123 }"
        )?,
        123
    );
    assert_eq!(
        engine.eval_with_scope::<char>(
            &mut scope,
            "switch 2 { 1 => (), 2 if x > 40 => 'a', 42 => true, _ => 123 }"
        )?,
        'a'
    );
    assert!(
        engine.eval_with_scope::<bool>(&mut scope, "switch x { 1 => (), 2 => 'a', 42 => true }")?
    );
    assert!(
        engine.eval_with_scope::<bool>(&mut scope, "switch x { 1 => (), 2 => 'a', _ => true }")?
    );
    let _: () = engine.eval_with_scope::<()>(&mut scope, "switch x { 1 => 123, 2 => 'a' }")?;

    assert_eq!(
        engine.eval_with_scope::<INT>(
            &mut scope,
            "switch x { 1 | 2 | 3 | 5..50 | 'x' | true => 123, 'z' => 'a' }"
        )?,
        123
    );
    assert_eq!(
        engine.eval_with_scope::<INT>(&mut scope, "switch x { 424242 => 123, _ => 42 }")?,
        42
    );
    assert_eq!(
        engine.eval_with_scope::<INT>(
            &mut scope,
            "switch x { 1 => 123, 42 => { x / 2 }, _ => 999 }"
        )?,
        21
    );
    #[cfg(not(feature = "no_index"))]
    assert_eq!(
        engine.eval_with_scope::<INT>(
            &mut scope,
            "
                let y = [1, 2, 3];

                switch y {
                    42 => 1,
                    true => 2,
                    [1, 2, 3] => 3,
                    _ => 9
                }
            "
        )?,
        3
    );
    #[cfg(not(feature = "no_object"))]
    assert_eq!(
        engine.eval_with_scope::<INT>(
            &mut scope,
            "
                let y = #{a:1, b:true, c:'x'};

                switch y {
                    42 => 1,
                    true => 2,
                    #{b:true, c:'x', a:1} => 3,
                    _ => 9
                }
            "
        )?,
        3
    );

    assert_eq!(
        engine.eval_with_scope::<INT>(&mut scope, "switch 42 { 42 => 123, 42 => 999 }")?,
        123
    );

    assert_eq!(
        engine.eval_with_scope::<INT>(&mut scope, "switch x { 42 => 123, 42 => 999 }")?,
        123
    );

    Ok(())
}

#[test]
fn test_switch_errors() -> Result<(), Box<EvalAltResult>> {
    let engine = Engine::new();

    assert!(matches!(
        engine
            .compile("switch x { _ => 123, 1 => 42 }")
            .unwrap_err()
            .err_type(),
        ParseErrorType::WrongSwitchDefaultCase
    ));

    Ok(())
}

#[test]
fn test_switch_condition() -> Result<(), Box<EvalAltResult>> {
    let engine = Engine::new();
    let mut scope = Scope::new();
    scope.push("x", 42 as INT);

    assert_eq!(
        engine.eval_with_scope::<INT>(
            &mut scope,
            "
                switch x / 2 {
                    21 if x > 40 => 1,
                    0 if x < 100 => 2,
                    1 => 3,
                    _ => 9
                }
            "
        )?,
        1
    );

    assert_eq!(
        engine.eval_with_scope::<INT>(
            &mut scope,
            "
                switch x / 2 {
                    21 if x < 40 => 1,
                    0 if x < 100 => 2,
                    1 => 3,
                    _ => 9
                }
            "
        )?,
        9
    );

    assert_eq!(
        engine.eval_with_scope::<INT>(
            &mut scope,
            "
                switch x {
                    42 if x < 40 => 1,
                    42 if x > 40 => 7,
                    0 if x < 100 => 2,
                    1 => 3,
                    42 if x == 10 => 10,
                    _ => 9
                }
            "
        )?,
        7
    );

    assert!(matches!(
        engine
            .compile("switch x { 1 => 123, _ if true => 42 }")
            .unwrap_err()
            .err_type(),
        ParseErrorType::WrongSwitchCaseCondition
    ));

    Ok(())
}

#[cfg(not(feature = "no_index"))]
#[cfg(not(feature = "no_object"))]
mod test_switch_enum {
    use super::*;
    use rhai::Array;
    #[derive(Debug, Clone)]
    #[allow(dead_code)]
    enum MyEnum {
        Foo,
        Bar(INT),
        Baz(String, bool),
    }

    impl MyEnum {
        fn get_enum_data(&mut self) -> Array {
            match self {
                Self::Foo => vec!["Foo".into()] as Array,
                Self::Bar(num) => vec!["Bar".into(), (*num).into()] as Array,
                Self::Baz(name, option) => {
                    vec!["Baz".into(), name.clone().into(), (*option).into()] as Array
                }
            }
        }
    }

    #[test]
    fn test_switch_enum() -> Result<(), Box<EvalAltResult>> {
        let mut engine = Engine::new();

        engine
            .register_type_with_name::<MyEnum>("MyEnum")
            .register_get("get_data", MyEnum::get_enum_data);

        let mut scope = Scope::new();
        scope.push("x", MyEnum::Baz("hello".to_string(), true));

        assert_eq!(
            engine.eval_with_scope::<INT>(
                &mut scope,
                r#"
                    switch x.get_data {
                        ["Foo"] => 1,
                        ["Bar", 42] => 2,
                        ["Bar", 123] => 3,
                        ["Baz", "hello", false] => 4,
                        ["Baz", "hello", true] => 5,
                        _ => 9
                    }
                "#
            )?,
            5
        );

        Ok(())
    }
}

#[test]
fn test_switch_ranges() -> Result<(), Box<EvalAltResult>> {
    let engine = Engine::new();
    let mut scope = Scope::new();
    scope.push("x", 42 as INT);

    assert_eq!(
        engine.eval_with_scope::<char>(
            &mut scope,
            "switch x { 10..20 => (), 20..=42 => 'a', 25..45 => 'z', 30..100 => true }"
        )?,
        'a'
    );
    assert_eq!(
        engine.eval_with_scope::<char>(
            &mut scope,
            "switch x { 10..20 => (), 20..=42 if x < 40 => 'a', 25..45 => 'z', 30..100 => true }"
        )?,
        'z'
    );
    assert_eq!(
        engine.eval_with_scope::<char>(
            &mut scope,
            "switch x { 42 => 'x', 10..20 => (), 20..=42 => 'a', 25..45 => 'z', 30..100 => true, 'w' => true }"
        )?,
        'x'
    );
    assert!(matches!(
        engine.compile(
            "switch x { 10..20 => (), 20..=42 => 'a', 25..45 => 'z', 42 => 'x', 30..100 => true }"
        ).unwrap_err().err_type(),
        ParseErrorType::WrongSwitchIntegerCase
    ));
    #[cfg(not(feature = "no_float"))]
    assert!(matches!(
        engine.compile(
            "switch x { 10..20 => (), 20..=42 => 'a', 25..45 => 'z', 42.0 => 'x', 30..100 => true }"
        ).unwrap_err().err_type(),
        ParseErrorType::WrongSwitchIntegerCase
    ));
    assert_eq!(
        engine.eval_with_scope::<char>(
            &mut scope,
            "
                switch 5 {
                    'a' => true,
                    0..10 if x+2==1+2 => print(40+2),
                    _ => 'x'
                }
            "
        )?,
        'x'
    );
    assert_eq!(
        engine.eval_with_scope::<INT>(
            &mut scope,
            "
                switch 5 {
                    'a' => true,
                    0..10 => 123,
                    2..12 => 'z',
                    _ => 'x'
                }
            "
        )?,
        123
    );
    assert_eq!(
        engine.eval_with_scope::<INT>(
            &mut scope,
            "
                switch 5 {
                    'a' => true,
                    4 | 5 | 6 => 42,
                    0..10 => 123,
                    2..12 => 'z',
                    _ => 'x'
                }
            "
        )?,
        42
    );
    assert_eq!(
        engine.eval_with_scope::<char>(
            &mut scope,
            "
                switch 5 {
                    'a' => true,
                    2..12 => 'z',
                    0..10 if x+2==1+2 => print(40+2),
                    _ => 'x'
                }
            "
        )?,
        'z'
    );
    assert_eq!(
        engine.eval_with_scope::<char>(
            &mut scope,
            "
                switch 5 {
                    'a' => true,
                    0..10 if x+2==1+2 => print(40+2),
                    2..12 => 'z',
                    _ => 'x'
                }
            "
        )?,
        'z'
    );

    Ok(())
}
