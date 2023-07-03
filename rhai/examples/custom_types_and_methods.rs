//! An example showing how to register a Rust type and methods/getters/setters for it.

#[cfg(feature = "no_object")]
fn main() {
    panic!("This example does not run under 'no_object'.");
}

use rhai::{Engine, EvalAltResult};

#[cfg(not(feature = "no_object"))]
fn main() -> Result<(), Box<EvalAltResult>> {
    #[derive(Debug, Clone)]
    struct TestStruct {
        x: i64,
    }

    impl TestStruct {
        pub fn new() -> Self {
            Self { x: 1 }
        }
        pub fn update(&mut self) {
            self.x += 1000;
        }
        pub fn calculate(&mut self, data: i64) -> i64 {
            self.x * data
        }
        pub fn get_x(&mut self) -> i64 {
            self.x
        }
        pub fn set_x(&mut self, value: i64) {
            self.x = value;
        }
    }

    let mut engine = Engine::new();

    engine
        .register_type_with_name::<TestStruct>("TestStruct")
        .register_fn("new_ts", TestStruct::new)
        .register_fn("update", TestStruct::update)
        .register_fn("calc", TestStruct::calculate)
        .register_get_set("x", TestStruct::get_x, TestStruct::set_x);

    #[cfg(feature = "metadata")]
    {
        println!("Functions registered:");

        engine
            .gen_fn_signatures(false)
            .into_iter()
            .for_each(|func| println!("{func}"));

        println!();
    }

    let result = engine.eval::<i64>(
        "
            let x = new_ts();
            x.x = 42;
            x.update();
            x.calc(x.x)
        ",
    )?;

    println!("result: {result}"); // prints 1085764

    Ok(())
}
