use rhai::plugin::*;
use rhai::{Engine, EvalAltResult, Module, FLOAT};

pub mod raw_fn {
    use rhai::plugin::*;
    use rhai::FLOAT;

    #[export_fn]
    pub fn distance_function(x1: FLOAT, y1: FLOAT, x2: FLOAT, y2: FLOAT) -> FLOAT {
        ((y2 - y1).abs().powf(2.0) + (x2 - x1).abs().powf(2.0)).sqrt()
    }
}

#[test]
fn raw_fn_test() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();
    engine.register_fn("get_mystic_number", || 42 as FLOAT);
    let mut m = Module::new();
    rhai::set_exported_fn!(m, "euclidean_distance", raw_fn::distance_function);
    engine.register_static_module("Math::Advanced", m.into());

    assert_eq!(
        engine.eval::<FLOAT>(
            r#"let x = Math::Advanced::euclidean_distance(0.0, 1.0, 0.0, get_mystic_number()); x"#
        )?,
        41.0
    );
    Ok(())
}

mod raw_fn_mut {
    use rhai::plugin::*;
    use rhai::FLOAT;

    #[export_fn]
    pub fn add_in_place(f1: &mut FLOAT, f2: FLOAT) {
        *f1 += f2;
    }
}

#[test]
fn raw_fn_mut_test() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();
    engine.register_fn("get_mystic_number", || 42 as FLOAT);
    let mut m = Module::new();
    rhai::set_exported_fn!(m, "add_in_place", raw_fn_mut::add_in_place);
    engine.register_static_module("Math::Advanced", m.into());

    assert_eq!(
        engine.eval::<FLOAT>(
            r#"let x = get_mystic_number();
            Math::Advanced::add_in_place(x, 1.0);
            x"#
        )?,
        43.0
    );
    Ok(())
}

mod raw_fn_str {
    use rhai::plugin::*;

    #[export_fn]
    pub fn write_out_str(message: &str) -> bool {
        eprintln!("{message}");
        true
    }
}

#[test]
fn raw_fn_str_test() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();
    engine.register_fn("get_mystic_number", || 42 as FLOAT);
    let mut m = Module::new();
    rhai::set_exported_fn!(m, "write_out_str", raw_fn_str::write_out_str);
    engine.register_static_module("Host::IO", m.into());

    assert!(engine
        .eval::<bool>(r#"let x = Host::IO::write_out_str("hello world!"); x"#)
        .unwrap());
    Ok(())
}

mod mut_opaque_ref {
    use rhai::plugin::*;
    use rhai::INT;

    #[allow(dead_code)] // used inside `export_module!`
    #[derive(Clone)]
    pub struct StatusMessage {
        os_code: Option<INT>,
        message: String,
        is_ok: bool,
    }

    #[export_fn]
    pub fn new_message(is_ok: bool, message: &str) -> StatusMessage {
        StatusMessage {
            is_ok,
            os_code: None,
            message: message.to_string(),
        }
    }

    #[export_fn]
    pub fn new_os_message(is_ok: bool, os_code: INT) -> StatusMessage {
        StatusMessage {
            is_ok,
            os_code: Some(os_code),
            message: format!("OS Code {os_code}"),
        }
    }

    #[export_fn]
    pub fn write_out_message(message: &mut StatusMessage) -> bool {
        eprintln!("{}", message.message);
        true
    }
}

#[test]
fn mut_opaque_ref_test() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();
    let mut m = Module::new();
    rhai::set_exported_fn!(m, "new_message", mut_opaque_ref::new_message);
    rhai::set_exported_fn!(m, "new_os_message", mut_opaque_ref::new_os_message);
    rhai::set_exported_fn!(m, "write_out_message", mut_opaque_ref::write_out_message);
    engine.register_static_module("Host::Msg", m.into());

    assert!(engine
        .eval::<bool>(
            r#"
            let message1 = Host::Msg::new_message(true, "it worked");
            let ok1 = Host::Msg::write_out_message(message1);
            let message2 = Host::Msg::new_os_message(true, 0);
            let ok2 = Host::Msg::write_out_message(message2);
            ok1 && ok2"#
        )
        .unwrap());
    Ok(())
}

pub mod raw_returning_fn {
    use rhai::plugin::*;
    use rhai::FLOAT;

    #[export_fn(return_raw)]
    pub fn distance_function(
        x1: FLOAT,
        y1: FLOAT,
        x2: FLOAT,
        y2: FLOAT,
    ) -> Result<rhai::FLOAT, Box<rhai::EvalAltResult>> {
        Ok(((y2 - y1).abs().powf(2.0) + (x2 - x1).abs().powf(2.0)).sqrt())
    }
}

#[test]
fn raw_returning_fn_test() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();
    engine.register_fn("get_mystic_number", || 42 as FLOAT);
    let mut m = Module::new();
    rhai::set_exported_fn!(m, "euclidean_distance", raw_returning_fn::distance_function);
    engine.register_static_module("Math::Advanced", m.into());

    assert_eq!(
        engine.eval::<FLOAT>(
            r#"let x = Math::Advanced::euclidean_distance(0.0, 1.0, 0.0, get_mystic_number()); x"#
        )?,
        41.0
    );
    Ok(())
}
