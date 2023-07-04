#![cfg(not(feature = "no_float"))]
use rhai::{Engine, EvalAltResult, FLOAT};

const EPSILON: FLOAT = 0.000_000_000_1;

#[test]
fn test_float() -> Result<(), Box<EvalAltResult>> {
    let engine = Engine::new();

    assert!(engine.eval::<bool>("let x = 0.0; let y = 1.0; x < y")?);
    assert!(!engine.eval::<bool>("let x = 0.0; let y = 1.0; x > y")?);
    assert!(!engine.eval::<bool>("let x = 0.; let y = 1.; x > y")?);
    assert!((engine.eval::<FLOAT>("let x = 9.9999; x")? - 9.9999 as FLOAT).abs() < EPSILON);

    Ok(())
}

#[test]
fn test_float_scientific() -> Result<(), Box<EvalAltResult>> {
    let engine = Engine::new();

    assert!(engine.eval::<bool>("123.456 == 1.23456e2")?);
    assert!(engine.eval::<bool>("123.456 == 1.23456e+2")?);
    assert!(engine.eval::<bool>("123.456 == 123456e-3")?);

    Ok(())
}

#[test]
fn test_float_parse() -> Result<(), Box<EvalAltResult>> {
    let engine = Engine::new();

    assert!((engine.eval::<FLOAT>(r#"parse_float("9.9999")"#)? - 9.9999 as FLOAT).abs() < EPSILON);

    Ok(())
}

#[test]
#[cfg(not(feature = "no_object"))]
fn test_struct_with_float() -> Result<(), Box<EvalAltResult>> {
    #[derive(Clone)]
    struct TestStruct {
        x: FLOAT,
    }

    impl TestStruct {
        fn update(&mut self) {
            self.x += 5.789;
        }

        fn get_x(&mut self) -> FLOAT {
            self.x
        }

        fn set_x(&mut self, new_x: FLOAT) {
            self.x = new_x;
        }

        fn new() -> Self {
            Self { x: 1.0 }
        }
    }

    let mut engine = Engine::new();

    engine.register_type::<TestStruct>();

    engine.register_get_set("x", TestStruct::get_x, TestStruct::set_x);
    engine.register_fn("update", TestStruct::update);
    engine.register_fn("new_ts", TestStruct::new);

    assert!(
        (engine.eval::<FLOAT>("let ts = new_ts(); ts.update(); ts.x")? - 6.789).abs() < EPSILON
    );
    assert!(
        (engine.eval::<FLOAT>("let ts = new_ts(); ts.x = 10.1001; ts.x")? - 10.1001).abs()
            < EPSILON
    );

    Ok(())
}

#[test]
fn test_float_func() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();

    engine.register_fn("sum", |x: FLOAT, y: FLOAT, z: FLOAT, w: FLOAT| {
        x + y + z + w
    });

    assert_eq!(engine.eval::<FLOAT>("sum(1.0, 2.0, 3.0, 4.0)")?, 10.0);

    Ok(())
}
