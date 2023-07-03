#![cfg(feature = "debugging")]
use rhai::{Dynamic, Engine, EvalAltResult, INT};

#[cfg(not(feature = "no_index"))]
use rhai::Array;

#[cfg(not(feature = "no_object"))]
use rhai::Map;

#[test]
fn test_debugging() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();

    engine.register_debugger(
        |_, dbg| dbg,
        |_, _, _, _, _| Ok(rhai::debugger::DebuggerCommand::Continue),
    );

    #[cfg(not(feature = "no_function"))]
    #[cfg(not(feature = "no_index"))]
    {
        let r = engine.eval::<Array>(
            "
                fn foo(x) {
                    if x >= 5 {
                        back_trace()
                    } else {
                        foo(x+1)
                    }
                }

                foo(0)
            ",
        )?;

        assert_eq!(r.len(), 6);

        assert_eq!(engine.eval::<INT>("len(back_trace())")?, 0);
    }

    Ok(())
}

#[test]
#[cfg(not(feature = "no_object"))]
fn test_debugger_state() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();

    engine.register_debugger(
        |_, mut debugger| {
            // Say, use an object map for the debugger state
            let mut state = Map::new();
            // Initialize properties
            state.insert("hello".into(), (42 as INT).into());
            state.insert("foo".into(), false.into());
            debugger.set_state(state);
            debugger
        },
        |mut context, _, _, _, _| {
            // Print debugger state - which is an object map
            println!(
                "Current state = {}",
                context.global_runtime_state().debugger().state()
            );

            // Modify state
            let mut state = context
                .global_runtime_state_mut()
                .debugger_mut()
                .state_mut()
                .write_lock::<Map>()
                .unwrap();
            let hello = state.get("hello").unwrap().as_int().unwrap();
            state.insert("hello".into(), (hello + 1).into());
            state.insert("foo".into(), true.into());
            state.insert("something_new".into(), "hello, world!".into());

            // Continue with debugging
            Ok(rhai::debugger::DebuggerCommand::StepInto)
        },
    );

    engine.run("let x = 42;")?;

    Ok(())
}
