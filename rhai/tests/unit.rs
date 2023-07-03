use rhai::{Engine, EvalAltResult};

#[test]
fn test_unit() -> Result<(), Box<EvalAltResult>> {
    let engine = Engine::new();
    engine.run("let x = (); x")?;
    Ok(())
}

#[test]
fn test_unit_eq() -> Result<(), Box<EvalAltResult>> {
    let engine = Engine::new();
    assert!(engine.eval::<bool>("let x = (); let y = (); x == y")?);
    Ok(())
}

#[test]
fn test_unit_with_spaces() -> Result<(), Box<EvalAltResult>> {
    let engine = Engine::new();
    let _ = engine.run("let x = ( ); x").unwrap_err();
    Ok(())
}
