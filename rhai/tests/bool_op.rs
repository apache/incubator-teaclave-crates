use rhai::{Engine, EvalAltResult};

#[test]
fn test_bool_op1() -> Result<(), Box<EvalAltResult>> {
    let engine = Engine::new();

    assert!(engine.eval::<bool>("true && (false || true)")?);
    assert!(engine.eval::<bool>("true & (false | true)")?);

    Ok(())
}

#[test]
fn test_bool_op2() -> Result<(), Box<EvalAltResult>> {
    let engine = Engine::new();

    assert!(!engine.eval::<bool>("false && (false || true)")?);
    assert!(!engine.eval::<bool>("false & (false | true)")?);

    Ok(())
}

#[test]
fn test_bool_op3() -> Result<(), Box<EvalAltResult>> {
    let engine = Engine::new();

    assert!(engine.eval::<bool>("true && (false || 123)").is_err());
    assert!(engine.eval::<bool>("true && (true || { throw })")?);
    assert!(engine.eval::<bool>("123 && (false || true)").is_err());
    assert!(!engine.eval::<bool>("false && (true || { throw })")?);

    Ok(())
}

#[test]
fn test_bool_op_short_circuit() -> Result<(), Box<EvalAltResult>> {
    let engine = Engine::new();

    assert!(engine.eval::<bool>(
        "
                let x = true;
                x || { throw; };
            "
    )?);

    assert!(!engine.eval::<bool>(
        "
                let x = false;
                x && { throw; };
            "
    )?);

    Ok(())
}

#[test]
fn test_bool_op_no_short_circuit1() {
    let engine = Engine::new();

    let _ = engine
        .eval::<bool>(
            "
                let x = true;
                x | { throw; }
            ",
        )
        .unwrap_err();
}

#[test]
fn test_bool_op_no_short_circuit2() {
    let engine = Engine::new();

    assert!(engine
        .eval::<bool>(
            "
                let x = false;
                x & { throw; }
            "
        )
        .is_err());
}
