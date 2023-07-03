// Copyright 2018 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Implementation for SGX using RDRAND instruction
extern crate sgx_trts;

use sgx_trts::rand::rand;

use core::mem::MaybeUninit;

use crate::{util::slice_as_uninit, Error};

// Vec can not be used under no-std, so we generate the random numbers by chunks
pub fn getrandom_inner(dest: &mut [MaybeUninit<u8>]) -> Result<(), Error> {
    const WINDOW_SIZE: usize = 8;
    let mut buf = [0; WINDOW_SIZE];

    let mut chunks = dest.chunks_exact_mut(WINDOW_SIZE);
    for chunk in chunks.by_ref() {
        rand(&mut buf).or(Err(Error::UNSUPPORTED))?;
        chunk.copy_from_slice(slice_as_uninit(&buf));
    }

    let tail = chunks.into_remainder();
    let n = tail.len();
    if n > 0 {
        rand(&mut buf).or(Err(Error::UNSUPPORTED))?;
        tail.copy_from_slice(slice_as_uninit(&buf[..n]));
    }

    Ok(())
}
