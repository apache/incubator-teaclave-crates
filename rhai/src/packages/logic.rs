use crate::def_package;
use crate::module::ModuleFlags;
use crate::plugin::*;
#[cfg(feature = "no_std")]
use std::prelude::v1::*;

#[cfg(any(
    not(feature = "no_float"),
    all(not(feature = "only_i32"), not(feature = "only_i64"))
))]
macro_rules! gen_cmp_functions {
    ($root:ident => $($arg_type:ident),+) => {
        mod $root { $(pub mod $arg_type {
            use super::super::*;

            #[export_module]
            pub mod functions {
                #[rhai_fn(name = "<")] pub fn lt(x: $arg_type, y: $arg_type) -> bool { x < y }
                #[rhai_fn(name = "<=")] pub fn lte(x: $arg_type, y: $arg_type) -> bool { x <= y }
                #[rhai_fn(name = ">")] pub fn gt(x: $arg_type, y: $arg_type) -> bool { x > y }
                #[rhai_fn(name = ">=")] pub fn gte(x: $arg_type, y: $arg_type) -> bool { x >= y }
                #[rhai_fn(name = "==")] pub fn eq(x: $arg_type, y: $arg_type) -> bool { x == y }
                #[rhai_fn(name = "!=")] pub fn ne(x: $arg_type, y: $arg_type) -> bool { x != y }
                pub fn max(x: $arg_type, y: $arg_type) -> $arg_type { if x >= y { x } else { y } }
                pub fn min(x: $arg_type, y: $arg_type) -> $arg_type { if x <= y { x } else { y } }
            }
        })* }
    };
}

#[cfg(any(
    not(feature = "no_float"),
    all(not(feature = "only_i32"), not(feature = "only_i64"))
))]
macro_rules! reg_functions {
    ($mod_name:ident += $root:ident ; $($arg_type:ident),+) => { $(
        combine_with_exported_module!($mod_name, "logic", $root::$arg_type::functions);
    )* }
}

def_package! {
    /// Package of basic logic operators.
    pub LogicPackage(lib) {
        lib.flags |= ModuleFlags::STANDARD_LIB;

        #[cfg(not(feature = "only_i32"))]
        #[cfg(not(feature = "only_i64"))]
        {
            reg_functions!(lib += numbers; i8, u8, i16, u16, i32, u32, u64);

            #[cfg(not(target_family = "wasm"))]

            reg_functions!(lib += num_128; i128, u128);
        }

        #[cfg(not(feature = "no_float"))]
        {
            combine_with_exported_module!(lib, "float", float_functions);

            #[cfg(not(feature = "f32_float"))]
            {
                reg_functions!(lib += float; f32);
                combine_with_exported_module!(lib, "f32", f32_functions);
            }
            #[cfg(feature = "f32_float")]
            {
                reg_functions!(lib += float; f64);
                combine_with_exported_module!(lib, "f64", f64_functions);
            }
        }

        #[cfg(feature = "decimal")]
        combine_with_exported_module!(lib, "decimal", decimal_functions);

        combine_with_exported_module!(lib, "logic", logic_functions);
    }
}

#[cfg(not(feature = "only_i32"))]
#[cfg(not(feature = "only_i64"))]
gen_cmp_functions!(numbers => i8, u8, i16, u16, i32, u32, u64);

#[cfg(not(feature = "only_i32"))]
#[cfg(not(feature = "only_i64"))]
#[cfg(not(target_family = "wasm"))]

gen_cmp_functions!(num_128 => i128, u128);

#[cfg(not(feature = "no_float"))]
#[cfg(not(feature = "f32_float"))]
gen_cmp_functions!(float => f32);

#[cfg(not(feature = "no_float"))]
#[cfg(feature = "f32_float")]
gen_cmp_functions!(float => f64);

#[export_module]
mod logic_functions {
    #[rhai_fn(name = "!")]
    pub fn not(x: bool) -> bool {
        !x
    }
}

#[cfg(not(feature = "no_float"))]
#[allow(clippy::cast_precision_loss)]
#[export_module]
mod float_functions {
    use crate::INT;

    #[rhai_fn(name = "max")]
    pub fn max_if_32(x: INT, y: f32) -> f32 {
        let (x, y) = (x as f32, y as f32);
        if x >= y {
            x
        } else {
            y
        }
    }
    #[rhai_fn(name = "max")]
    pub fn max_fi_32(x: f32, y: INT) -> f32 {
        let (x, y) = (x as f32, y as f32);
        if x >= y {
            x
        } else {
            y
        }
    }
    #[rhai_fn(name = "min")]
    pub fn min_if_32(x: INT, y: f32) -> f32 {
        let (x, y) = (x as f32, y as f32);
        if x <= y {
            x
        } else {
            y
        }
    }
    #[rhai_fn(name = "min")]
    pub fn min_fi_32(x: f32, y: INT) -> f32 {
        let (x, y) = (x as f32, y as f32);
        if x <= y {
            x
        } else {
            y
        }
    }
    #[rhai_fn(name = "max")]
    pub fn max_if_64(x: INT, y: f64) -> f64 {
        let (x, y) = (x as f64, y as f64);
        if x >= y {
            x
        } else {
            y
        }
    }
    #[rhai_fn(name = "max")]
    pub fn max_fi_64(x: f64, y: INT) -> f64 {
        let (x, y) = (x as f64, y as f64);
        if x >= y {
            x
        } else {
            y
        }
    }
    #[rhai_fn(name = "min")]
    pub fn min_if_64(x: INT, y: f64) -> f64 {
        let (x, y) = (x as f64, y as f64);
        if x <= y {
            x
        } else {
            y
        }
    }
    #[rhai_fn(name = "min")]
    pub fn min_fi_64(x: f64, y: INT) -> f64 {
        let (x, y) = (x as f64, y as f64);
        if x <= y {
            x
        } else {
            y
        }
    }
}

#[cfg(not(feature = "no_float"))]
#[cfg(not(feature = "f32_float"))]
#[allow(clippy::cast_precision_loss)]
#[export_module]
mod f32_functions {
    use crate::{FLOAT, INT};

    #[rhai_fn(name = "==")]
    pub fn eq_if(x: INT, y: f32) -> bool {
        (x as f32) == (y as f32)
    }
    #[rhai_fn(name = "==")]
    pub fn eq_fi(x: f32, y: INT) -> bool {
        (x as f32) == (y as f32)
    }
    #[rhai_fn(name = "!=")]
    pub fn neq_if(x: INT, y: f32) -> bool {
        (x as f32) != (y as f32)
    }
    #[rhai_fn(name = "!=")]
    pub fn neq_fi(x: f32, y: INT) -> bool {
        (x as f32) != (y as f32)
    }
    #[rhai_fn(name = ">")]
    pub fn gt_if(x: INT, y: f32) -> bool {
        (x as f32) > (y as f32)
    }
    #[rhai_fn(name = ">")]
    pub fn gt_fi(x: f32, y: INT) -> bool {
        (x as f32) > (y as f32)
    }
    #[rhai_fn(name = ">=")]
    pub fn gte_if(x: INT, y: f32) -> bool {
        (x as f32) >= (y as f32)
    }
    #[rhai_fn(name = ">=")]
    pub fn gte_fi(x: f32, y: INT) -> bool {
        (x as f32) >= (y as f32)
    }
    #[rhai_fn(name = "<")]
    pub fn lt_if(x: INT, y: f32) -> bool {
        (x as f32) < (y as f32)
    }
    #[rhai_fn(name = "<")]
    pub fn lt_fi(x: f32, y: INT) -> bool {
        (x as f32) < (y as f32)
    }
    #[rhai_fn(name = "<=")]
    pub fn lte_if(x: INT, y: f32) -> bool {
        (x as f32) <= (y as f32)
    }
    #[rhai_fn(name = "<=")]
    pub fn lte_fi(x: f32, y: INT) -> bool {
        (x as f32) <= (y as f32)
    }

    #[rhai_fn(name = "max")]
    pub fn max_64_32(x: FLOAT, y: f32) -> FLOAT {
        let (x, y) = (x as FLOAT, y as FLOAT);
        if x >= y {
            x
        } else {
            y
        }
    }
    #[rhai_fn(name = "max")]
    pub fn max_32_64(x: f32, y: FLOAT) -> FLOAT {
        let (x, y) = (x as FLOAT, y as FLOAT);
        if x >= y {
            x
        } else {
            y
        }
    }
    #[rhai_fn(name = "min")]
    pub fn min_64_32(x: FLOAT, y: f32) -> FLOAT {
        let (x, y) = (x as FLOAT, y as FLOAT);
        if x <= y {
            x
        } else {
            y
        }
    }
    #[rhai_fn(name = "min")]
    pub fn min_32_64(x: f32, y: FLOAT) -> FLOAT {
        let (x, y) = (x as FLOAT, y as FLOAT);
        if x <= y {
            x
        } else {
            y
        }
    }
}

#[cfg(not(feature = "no_float"))]
#[cfg(feature = "f32_float")]
#[allow(clippy::cast_precision_loss)]
#[export_module]
mod f64_functions {
    use crate::{FLOAT, INT};

    #[rhai_fn(name = "==")]
    pub fn eq_if(x: INT, y: f64) -> bool {
        (x as f64) == (y as f64)
    }
    #[rhai_fn(name = "==")]
    pub fn eq_fi(x: f64, y: INT) -> bool {
        (x as f64) == (y as f64)
    }
    #[rhai_fn(name = "!=")]
    pub fn neq_if(x: INT, y: f64) -> bool {
        (x as f64) != (y as f64)
    }
    #[rhai_fn(name = "!=")]
    pub fn neq_fi(x: f64, y: INT) -> bool {
        (x as f64) != (y as f64)
    }
    #[rhai_fn(name = ">")]
    pub fn gt_if(x: INT, y: f64) -> bool {
        (x as f64) > (y as f64)
    }
    #[rhai_fn(name = ">")]
    pub fn gt_fi(x: f64, y: INT) -> bool {
        (x as f64) > (y as f64)
    }
    #[rhai_fn(name = ">=")]
    pub fn gte_if(x: INT, y: f64) -> bool {
        (x as f64) >= (y as f64)
    }
    #[rhai_fn(name = ">=")]
    pub fn gte_fi(x: f64, y: INT) -> bool {
        (x as f64) >= (y as f64)
    }
    #[rhai_fn(name = "<")]
    pub fn lt_if(x: INT, y: f64) -> bool {
        (x as f64) < (y as f64)
    }
    #[rhai_fn(name = "<")]
    pub fn lt_fi(x: f64, y: INT) -> bool {
        (x as f64) < (y as f64)
    }
    #[rhai_fn(name = "<=")]
    pub fn lte_if(x: INT, y: f64) -> bool {
        (x as f64) <= (y as f64)
    }
    #[rhai_fn(name = "<=")]
    pub fn lte_fi(x: f64, y: INT) -> bool {
        (x as f64) <= (y as f64)
    }

    #[rhai_fn(name = "max")]
    pub fn max_32_64(x: FLOAT, y: f64) -> FLOAT {
        let (x, y) = (x as FLOAT, y as FLOAT);
        if x >= y {
            x
        } else {
            y
        }
    }
    #[rhai_fn(name = "max")]
    pub fn max_64_32(x: f64, y: FLOAT) -> FLOAT {
        let (x, y) = (x as FLOAT, y as FLOAT);
        if x >= y {
            x
        } else {
            y
        }
    }
    #[rhai_fn(name = "min")]
    pub fn min_32_64(x: FLOAT, y: f64) -> FLOAT {
        let (x, y) = (x as FLOAT, y as FLOAT);
        if x <= y {
            x
        } else {
            y
        }
    }
    #[rhai_fn(name = "min")]
    pub fn min_64_32(x: f64, y: FLOAT) -> FLOAT {
        let (x, y) = (x as FLOAT, y as FLOAT);
        if x <= y {
            x
        } else {
            y
        }
    }
}

#[cfg(feature = "decimal")]
#[export_module]
mod decimal_functions {
    use crate::INT;
    use rust_decimal::Decimal;

    #[rhai_fn(name = "max")]
    pub fn max_id(x: INT, y: Decimal) -> Decimal {
        let x = x.into();
        if x >= y {
            x
        } else {
            y
        }
    }
    #[rhai_fn(name = "max")]
    pub fn max_di(x: Decimal, y: INT) -> Decimal {
        let y = y.into();
        if x >= y {
            x
        } else {
            y
        }
    }
    #[rhai_fn(name = "min")]
    pub fn min_id(x: INT, y: Decimal) -> Decimal {
        let x = x.into();
        if x <= y {
            x
        } else {
            y
        }
    }
    #[rhai_fn(name = "min")]
    pub fn min_di(x: Decimal, y: INT) -> Decimal {
        let y = y.into();
        if x <= y {
            x
        } else {
            y
        }
    }
}
