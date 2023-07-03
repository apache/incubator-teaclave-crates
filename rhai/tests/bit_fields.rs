use rhai::{Engine, EvalAltResult, INT};

#[test]
fn test_left_shift() -> Result<(), Box<EvalAltResult>> {
    let engine = Engine::new();
    assert_eq!(engine.eval::<INT>("4 << 2")?, 16);
    Ok(())
}

#[test]
fn test_right_shift() -> Result<(), Box<EvalAltResult>> {
    let engine = Engine::new();
    assert_eq!(engine.eval::<INT>("9 >> 1")?, 4);
    Ok(())
}

#[cfg(not(feature = "no_index"))]
#[test]
fn test_bit_fields() -> Result<(), Box<EvalAltResult>> {
    let engine = Engine::new();
    assert!(!engine.eval::<bool>("let x = 10; x[0]")?);
    assert!(engine.eval::<bool>("let x = 10; x[1]")?);
    assert!(!engine.eval::<bool>("let x = 10; x[-1]")?);
    assert_eq!(
        engine.eval::<INT>("let x = 10; x[0] = true; x[1] = false; x")?,
        9
    );
    assert_eq!(engine.eval::<INT>("let x = 10; get_bits(x, 1, 3)")?, 5);
    assert_eq!(engine.eval::<INT>("let x = 10; x[1..=3]")?, 5);
    assert!(engine.eval::<INT>("let x = 10; x[1..99]").is_err());
    assert!(engine.eval::<INT>("let x = 10; x[-1..3]").is_err());
    assert_eq!(
        engine.eval::<INT>("let x = 10; set_bits(x, 1, 3, 7); x")?,
        14
    );
    #[cfg(target_pointer_width = "64")]
    #[cfg(not(feature = "only_i32"))]
    {
        assert_eq!(engine.eval::<INT>("let x = 255; get_bits(x, -60, 2)")?, 3);
        assert_eq!(
            engine.eval::<INT>("let x = 0; set_bits(x, -64, 1, 15); x")?,
            1
        );
        assert_eq!(
            engine.eval::<INT>("let x = 0; set_bits(x, -60, 2, 15); x")?,
            0b00110000
        );
    }
    assert_eq!(engine.eval::<INT>("let x = 10; x[1..4] = 7; x")?, 14);
    assert_eq!(
        engine.eval::<INT>(
            "
                let x = 0b001101101010001;
                let count = 0;

                for b in bits(x, 2, 10) {
                    if b { count += 1; }
                }

                count
            "
        )?,
        5
    );
    assert_eq!(
        engine.eval::<INT>(
            "
                let x = 0b001101101010001;
                let count = 0;

                for b in bits(x, 2..=11) {
                    if b { count += 1; }
                }

                count
            "
        )?,
        5
    );

    Ok(())
}
