#![cfg(not(feature = "no_index"))]
use rhai::{Array, Dynamic, Engine, EvalAltResult, ParseErrorType, INT};
use std::iter::FromIterator;

#[test]
fn test_arrays() -> Result<(), Box<EvalAltResult>> {
    let a = Array::from_iter([(42 as INT).into()]);

    assert_eq!(a[0].clone_cast::<INT>(), 42);

    let engine = Engine::new();

    assert_eq!(engine.eval::<INT>("let x = [1, 2, 3]; x[1]")?, 2);
    assert_eq!(engine.eval::<INT>("let x = [1, 2, 3,]; x[1]")?, 2);
    assert_eq!(engine.eval::<INT>("let y = [1, 2, 3]; y[1] = 5; y[1]")?, 5);
    assert_eq!(
        engine.eval::<char>(r#"let y = [1, [ 42, 88, "93" ], 3]; y[1][2][1]"#)?,
        '3'
    );
    assert_eq!(engine.eval::<INT>("let y = [1, 2, 3]; y[0]")?, 1);
    assert_eq!(engine.eval::<INT>("let y = [1, 2, 3]; y[-1]")?, 3);
    assert_eq!(engine.eval::<INT>("let y = [1, 2, 3]; y[-3]")?, 1);
    assert!(engine.eval::<bool>("let y = [1, 2, 3]; 2 in y")?);
    assert!(engine.eval::<bool>("let y = [1, 2, 3]; 42 !in y")?);
    assert_eq!(engine.eval::<INT>("let y = [1, 2, 3]; y += 4; y[3]")?, 4);
    assert_eq!(
        engine.eval::<INT>("let y = [1, 2, 3]; pad(y, 5, 42); len(y)")?,
        5
    );
    assert_eq!(
        engine.eval::<INT>("let y = [1, 2, 3]; pad(y, 5, [42]); len(y)")?,
        5
    );
    assert_eq!(
        engine.eval::<INT>("let y = [1, 2, 3]; pad(y, 5, [42, 999, 123]); y[4][0]")?,
        42
    );
    assert_eq!(
        engine
            .eval::<Dynamic>("let y = [1, 2, 3]; y[1] += 4; y")?
            .into_typed_array::<INT>()?,
        [1, 6, 3]
    );
    assert_eq!(
        engine
            .eval::<Dynamic>("let y = [1, 2, 3]; extract(y, 1, 10)")?
            .into_typed_array::<INT>()?,
        vec![2, 3]
    );
    assert_eq!(
        engine
            .eval::<Dynamic>("let y = [1, 2, 3]; extract(y, -3, 1)")?
            .into_typed_array::<INT>()?,
        vec![1]
    );
    assert_eq!(
        engine
            .eval::<Dynamic>("let y = [1, 2, 3]; extract(y, -99, 2)")?
            .into_typed_array::<INT>()?,
        vec![1, 2]
    );
    assert_eq!(
        engine
            .eval::<Dynamic>("let y = [1, 2, 3]; extract(y, 99, 1)")?
            .into_typed_array::<INT>()?,
        vec![] as Vec<INT>
    );

    #[cfg(not(feature = "no_object"))]
    {
        assert_eq!(
            engine
                .eval::<Dynamic>("let y = [1, 2, 3]; y.push(4); y")?
                .into_typed_array::<INT>()?,
            [1, 2, 3, 4]
        );
        assert_eq!(
            engine
                .eval::<Dynamic>("let y = [1, 2, 3]; y.insert(0, 4); y")?
                .into_typed_array::<INT>()?,
            [4, 1, 2, 3]
        );
        assert_eq!(
            engine
                .eval::<Dynamic>("let y = [1, 2, 3]; y.insert(999, 4); y")?
                .into_typed_array::<INT>()?,
            [1, 2, 3, 4]
        );
        assert_eq!(
            engine
                .eval::<Dynamic>("let y = [1, 2, 3]; y.insert(-2, 4); y")?
                .into_typed_array::<INT>()?,
            [1, 4, 2, 3]
        );
        assert_eq!(
            engine
                .eval::<Dynamic>("let y = [1, 2, 3]; y.insert(-999, 4); y")?
                .into_typed_array::<INT>()?,
            [4, 1, 2, 3]
        );
        assert_eq!(
            engine.eval::<INT>("let y = [1, 2, 3]; let z = [42]; y[z.len]")?,
            2
        );
        assert_eq!(
            engine.eval::<INT>("let y = [1, 2, [3, 4, 5, 6]]; let z = [42]; y[2][z.len]")?,
            4
        );
        assert_eq!(
            engine.eval::<INT>("let y = [1, 2, 3]; let z = [2]; y[z[0]]")?,
            3
        );

        assert_eq!(
            engine
                .eval::<Dynamic>(
                    "
                        let x = [2, 9];
                        x.insert(-1, 1);
                        x.insert(999, 3);
                        x.insert(-9, 99);

                        let r = x.remove(2);

                        let y = [4, 5];
                        x.append(y);

                        x
                    "
                )?
                .into_typed_array::<INT>()?,
            [99, 2, 9, 3, 4, 5]
        );
    }

    #[cfg(not(feature = "no_object"))]
    assert_eq!(
        engine.eval::<INT>(
            "
                let x = #{ foo: 42 };
                let n = 0;
                let a = [[x]];
                let i = [n];
                a[n][i[n]].foo
            "
        )?,
        42
    );

    assert_eq!(
        engine
            .eval::<Dynamic>(
                "
                    let x = [1, 2, 3];
                    x += [4, 5];
                    x
                "
            )?
            .into_typed_array::<INT>()?,
        [1, 2, 3, 4, 5]
    );
    assert_eq!(
        engine
            .eval::<Dynamic>(
                "
                    let x = [1, 2, 3];
                    let y = [4, 5];
                    x + y
                "
            )?
            .into_typed_array::<INT>()?,
        [1, 2, 3, 4, 5]
    );
    #[cfg(not(feature = "no_closure"))]
    assert!(!engine.eval::<bool>(
        "
            let x = 42;
            let y = [];
            let f = || x;
            for n in 0..10 {
                y += x;
            }
            some(y, |x| is_shared(x))
        "
    )?);

    let value = vec![
        String::from("hello"),
        String::from("world"),
        String::from("foo"),
        String::from("bar"),
    ];

    let array: Dynamic = value.into();

    assert_eq!(array.type_name(), "array");

    let array = array.cast::<Array>();

    assert_eq!(array[0].type_name(), "string");
    assert_eq!(array.len(), 4);

    Ok(())
}

#[cfg(not(feature = "no_float"))]
#[cfg(not(feature = "no_object"))]
#[test]
fn test_array_chaining() -> Result<(), Box<EvalAltResult>> {
    let engine = Engine::new();

    assert!(engine.eval::<bool>(
        "
            let v = [ PI() ];
            ( v[0].cos() ).sin() == v[0].cos().sin()
        "
    )?);

    Ok(())
}

#[test]
fn test_array_index_types() -> Result<(), Box<EvalAltResult>> {
    let engine = Engine::new();

    engine.compile("[1, 2, 3][0]['x']")?;

    assert!(matches!(
        engine.compile("[1, 2, 3]['x']").unwrap_err().err_type(),
        ParseErrorType::MalformedIndexExpr(..)
    ));

    #[cfg(not(feature = "no_float"))]
    assert!(matches!(
        engine.compile("[1, 2, 3][123.456]").unwrap_err().err_type(),
        ParseErrorType::MalformedIndexExpr(..)
    ));

    assert!(matches!(
        engine.compile("[1, 2, 3][()]").unwrap_err().err_type(),
        ParseErrorType::MalformedIndexExpr(..)
    ));

    assert!(matches!(
        engine
            .compile(r#"[1, 2, 3]["hello"]"#)
            .unwrap_err()
            .err_type(),
        ParseErrorType::MalformedIndexExpr(..)
    ));

    assert!(matches!(
        engine
            .compile("[1, 2, 3][true && false]")
            .unwrap_err()
            .err_type(),
        ParseErrorType::MalformedIndexExpr(..)
    ));

    Ok(())
}

#[test]
#[cfg(not(feature = "no_object"))]
fn test_array_with_structs() -> Result<(), Box<EvalAltResult>> {
    #[derive(Clone)]
    struct TestStruct {
        x: INT,
    }

    impl TestStruct {
        fn update(&mut self) {
            self.x += 1000;
        }

        fn get_x(&mut self) -> INT {
            self.x
        }

        fn set_x(&mut self, new_x: INT) {
            self.x = new_x;
        }

        fn new() -> Self {
            Self { x: 1 }
        }
    }

    let mut engine = Engine::new();

    engine.register_type::<TestStruct>();

    engine.register_get_set("x", TestStruct::get_x, TestStruct::set_x);
    engine.register_fn("update", TestStruct::update);
    engine.register_fn("new_ts", TestStruct::new);

    assert_eq!(engine.eval::<INT>("let a = [new_ts()]; a[0].x")?, 1);

    assert_eq!(
        engine.eval::<INT>(
            "
                let a = [new_ts()];
                a[0].x = 100;
                a[0].update();
                a[0].x
            "
        )?,
        1100
    );

    Ok(())
}

#[cfg(not(feature = "no_object"))]
#[cfg(not(feature = "no_function"))]
#[cfg(not(feature = "no_closure"))]
#[test]
fn test_arrays_map_reduce() -> Result<(), Box<EvalAltResult>> {
    let engine = Engine::new();

    assert_eq!(engine.eval::<INT>("[1].map(|x| x + 41)[0]")?, 42);
    assert_eq!(engine.eval::<INT>("[1].map(|| this + 41)[0]")?, 42);
    assert_eq!(
        engine.eval::<INT>("let x = [1, 2, 3]; x.for_each(|| this += 41); x[0]")?,
        42
    );
    assert_eq!(
        engine.eval::<INT>(
            "
                let x = [1, 2, 3];
                let sum = 0;
                let factor = 2;
                x.for_each(|| sum += this * factor);
                sum
            "
        )?,
        12
    );
    assert_eq!(engine.eval::<INT>("([1].map(|x| x + 41))[0]")?, 42);
    assert_eq!(
        engine.eval::<INT>("let c = 40; let y = 1; [1].map(|x, i| c + x + y + i)[0]")?,
        42
    );
    assert_eq!(
        engine.eval::<INT>("let x = [1, 2, 3]; x.for_each(|i| this += i); x[2]")?,
        5
    );

    assert_eq!(
        engine
            .eval::<Dynamic>(
                "
                    let x = [1, 2, 3];
                    x.filter(|v| v > 2)
                "
            )?
            .into_typed_array::<INT>()?,
        [3]
    );

    assert_eq!(
        engine
            .eval::<Dynamic>(
                "
                    let x = [1, 2, 3];
                    x.filter(|| this > 2)
                "
            )?
            .into_typed_array::<INT>()?,
        [3]
    );

    assert_eq!(
        engine
            .eval::<Dynamic>(
                "
                    let x = [1, 2, 3];
                    x.filter(|v, i| v > i)
                "
            )?
            .into_typed_array::<INT>()?,
        [1, 2, 3]
    );

    assert_eq!(
        engine
            .eval::<Dynamic>(
                "
                    let x = [1, 2, 3];
                    x.map(|v| v * 2)
                "
            )?
            .into_typed_array::<INT>()?,
        [2, 4, 6]
    );

    assert_eq!(
        engine
            .eval::<Dynamic>(
                "
                    let x = [1, 2, 3];
                    x.map(|| this * 2)
                "
            )?
            .into_typed_array::<INT>()?,
        [2, 4, 6]
    );

    assert_eq!(
        engine
            .eval::<Dynamic>(
                "
                    let x = [1, 2, 3];
                    x.map(|v, i| v * i)
                "
            )?
            .into_typed_array::<INT>()?,
        [0, 2, 6]
    );

    assert_eq!(
        engine.eval::<INT>(
            r#"
                let x = [1, 2, 3];
                x.reduce(|sum, v| if sum.type_of() == "()" { v * v } else { sum + v * v })
            "#
        )?,
        14
    );

    // assert_eq!(
    //     engine.eval::<INT>(
    //         "
    //             let x = [1, 2, 3];
    //             x.reduce(|sum, v, i| {
    //                 if i == 0 { sum = 10 }
    //                 sum + v * v
    //             })
    //         "
    //     )?,
    //     24
    // );

    // assert_eq!(
    //     engine.eval::<INT>(
    //         "
    //             let x = [1, 2, 3];
    //             x.reduce(|sum, i| {
    //                 if i == 0 { sum = 10 }
    //                 sum + this * this
    //             })
    //         "
    //     )?,
    //     24
    // );

    assert_eq!(
        engine.eval::<INT>(
            r#"
                let x = [1, 2, 3];
                x.reduce_rev(|sum, v| if sum.type_of() == "()" { v * v } else { sum + v * v })
            "#
        )?,
        14
    );

    assert_eq!(
        engine.eval::<INT>(
            "
                let x = [1, 2, 3];
                x.reduce_rev(|sum, v, i| { if i == 2 { sum = 10 } sum + v * v })
            "
        )?,
        24
    );

    assert!(engine.eval::<bool>(
        "
            let x = [1, 2, 3];
            x.some(|v| v > 1)
        "
    )?);

    assert!(engine.eval::<bool>(
        "
            let x = [1, 2, 3];
            x.some(|v, i| v * i == 0)
        "
    )?);

    assert!(!engine.eval::<bool>(
        "
            let x = [1, 2, 3];
            x.all(|v| v > 1)
        "
    )?);

    assert!(engine.eval::<bool>(
        "
            let x = [1, 2, 3];
            x.all(|v, i| v > i)
        "
    )?);

    assert_eq!(
        engine.eval::<INT>(
            "
                let x = [1, 2, 3];
                x.find(|v| v > 2)
            "
        )?,
        3
    );

    assert_eq!(
        engine.eval::<INT>(
            "
                let x = [1, 2, 3];
                x.find(|v, i| v * i == 6)
            "
        )?,
        3
    );

    engine.eval::<()>(
        "
            let x = [1, 2, 3, 2, 1];
            x.find(|v| v > 4)
        ",
    )?;

    assert_eq!(
        engine.eval::<INT>(
            "
                let x = [#{alice: 1}, #{bob: 2}, #{clara: 3}];
                x.find_map(|v| v.bob)
            "
        )?,
        2
    );

    engine.eval::<()>(
        "
            let x = [#{alice: 1}, #{bob: 2}, #{clara: 3}];
            x.find_map(|v| v.dave)
        ",
    )?;

    Ok(())
}

#[test]
fn test_arrays_elvis() -> Result<(), Box<EvalAltResult>> {
    let engine = Engine::new();

    engine.eval::<()>("let x = (); x?[2]")?;

    engine.run("let x = (); x?[2] = 42")?;

    Ok(())
}
