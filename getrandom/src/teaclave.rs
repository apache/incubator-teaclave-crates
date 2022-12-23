// Copyright 2018 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Implementation for SGX using RDRAND instruction
extern crate sgx_trts;

use crate::Error;

pub fn getrandom_inner(dest: &mut [u8]) -> Result<(), Error> {
    // sgx_read_rand cannot take len=0, but this function does
    if dest.is_empty() {
        return Ok(());
    }

    sgx_trts::rand::rand(dest).or(Err(Error::UNSUPPORTED))
}
