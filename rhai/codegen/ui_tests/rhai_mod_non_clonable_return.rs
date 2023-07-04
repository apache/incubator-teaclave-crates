use rhai::plugin::*;

struct NonClonable {
    a: f32,
    b: u32,
    c: char,
    d: bool,
}

#[export_module]
pub mod test_mod {
    pub fn test_fn(input: f32) -> NonClonable {
        NonClonable {
            a: input,
            b: 10,
            c: 'a',
            d: true,
        }
    }
}

fn main() {
    let n = test_mod::test_fn(20.0);
    if n.c == 'a' {
        println!("yes");
    } else {
        println!("no");
    }
}
