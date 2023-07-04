use crate::eval::calc_index;
use crate::module::ModuleFlags;
use crate::plugin::*;
use crate::{
    def_package, ExclusiveRange, InclusiveRange, Position, RhaiResultOf, ERR, INT, INT_BITS,
    UNSIGNED_INT,
};
#[cfg(feature = "no_std")]
use std::prelude::v1::*;

def_package! {
    /// Package of basic bit-field utilities.
    pub BitFieldPackage(lib) {
        lib.flags |= ModuleFlags::STANDARD_LIB;

        combine_with_exported_module!(lib, "bit_field", bit_field_functions);
    }
}

#[export_module]
mod bit_field_functions {
    /// Return `true` if the specified `bit` in the number is set.
    ///
    /// If `bit` < 0, position counts from the MSB (Most Significant Bit).
    ///
    /// # Example
    ///
    /// ```rhai
    /// let x = 123456;
    ///
    /// print(x.get_bit(5));    // prints false
    ///
    /// print(x.get_bit(6));    // prints true
    ///
    /// print(x.get_bit(-48));  // prints true on 64-bit
    /// ```
    #[rhai_fn(return_raw)]
    pub fn get_bit(value: INT, bit: INT) -> RhaiResultOf<bool> {
        let bit = calc_index(INT_BITS, bit, true, || {
            ERR::ErrorBitFieldBounds(INT_BITS, bit, Position::NONE).into()
        })?;

        Ok((value & (1 << bit)) != 0)
    }
    /// Set the specified `bit` in the number if the new value is `true`.
    /// Clear the `bit` if the new value is `false`.
    ///
    /// If `bit` < 0, position counts from the MSB (Most Significant Bit).
    ///
    /// # Example
    ///
    /// ```rhai
    /// let x = 123456;
    ///
    /// x.set_bit(5, true);
    ///
    /// print(x);               // prints 123488
    ///
    /// x.set_bit(6, false);
    ///
    /// print(x);               // prints 123424
    ///
    /// x.set_bit(-48, false);
    ///
    /// print(x);               // prints 57888 on 64-bit
    /// ```
    #[rhai_fn(return_raw)]
    pub fn set_bit(value: &mut INT, bit: INT, new_value: bool) -> RhaiResultOf<()> {
        let bit = calc_index(INT_BITS, bit, true, || {
            ERR::ErrorBitFieldBounds(INT_BITS, bit, Position::NONE).into()
        })?;

        let mask = 1 << bit;
        if new_value {
            *value |= mask;
        } else {
            *value &= !mask;
        }

        Ok(())
    }
    /// Return an exclusive range of bits in the number as a new number.
    ///
    /// # Example
    ///
    /// ```rhai
    /// let x = 123456;
    ///
    /// print(x.get_bits(5..10));       // print 18
    /// ```
    #[rhai_fn(name = "get_bits", return_raw)]
    pub fn get_bits_range(value: INT, range: ExclusiveRange) -> RhaiResultOf<INT> {
        let from = INT::max(range.start, 0);
        let to = INT::max(range.end, from);
        get_bits(value, from, to - from)
    }
    /// Return an inclusive range of bits in the number as a new number.
    ///
    /// # Example
    ///
    /// ```rhai
    /// let x = 123456;
    ///
    /// print(x.get_bits(5..=9));       // print 18
    /// ```
    #[rhai_fn(name = "get_bits", return_raw)]
    pub fn get_bits_range_inclusive(value: INT, range: InclusiveRange) -> RhaiResultOf<INT> {
        let from = INT::max(*range.start(), 0);
        let to = INT::max(*range.end(), from - 1);
        get_bits(value, from, to - from + 1)
    }
    /// Return a portion of bits in the number as a new number.
    ///
    /// * If `start` < 0, position counts from the MSB (Most Significant Bit).
    /// * If `bits` ≤ 0, zero is returned.
    /// * If `start` position + `bits` ≥ total number of bits, the bits after the `start` position are returned.
    ///
    /// # Example
    ///
    /// ```rhai
    /// let x = 123456;
    ///
    /// print(x.get_bits(5, 8));        // print 18
    /// ```
    #[rhai_fn(return_raw)]
    pub fn get_bits(value: INT, start: INT, bits: INT) -> RhaiResultOf<INT> {
        if bits <= 0 {
            return Ok(0);
        }

        let bit = calc_index(INT_BITS, start, true, || {
            ERR::ErrorBitFieldBounds(INT_BITS, start, Position::NONE).into()
        })?;

        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        let bits = if bit + bits as usize > INT_BITS {
            INT_BITS - bit
        } else {
            bits as usize
        };

        if bit == 0 && bits == INT_BITS {
            return Ok(value);
        }

        // 2^bits - 1
        #[allow(clippy::cast_possible_truncation)]
        let mask = ((2 as UNSIGNED_INT).pow(bits as u32) - 1) as INT;

        Ok(((value & (mask << bit)) >> bit) & mask)
    }
    /// Replace an exclusive range of bits in the number with a new value.
    ///
    /// # Example
    ///
    /// ```rhai
    /// let x = 123456;
    ///
    /// x.set_bits(5..10, 42);
    ///
    /// print(x);           // print 123200
    /// ```
    #[rhai_fn(name = "set_bits", return_raw)]
    pub fn set_bits_range(
        value: &mut INT,
        range: ExclusiveRange,
        new_value: INT,
    ) -> RhaiResultOf<()> {
        let from = INT::max(range.start, 0);
        let to = INT::max(range.end, from);
        set_bits(value, from, to - from, new_value)
    }
    /// Replace an inclusive range of bits in the number with a new value.
    ///
    /// # Example
    ///
    /// ```rhai
    /// let x = 123456;
    ///
    /// x.set_bits(5..=9, 42);
    ///
    /// print(x);           // print 123200
    /// ```
    #[rhai_fn(name = "set_bits", return_raw)]
    pub fn set_bits_range_inclusive(
        value: &mut INT,
        range: InclusiveRange,
        new_value: INT,
    ) -> RhaiResultOf<()> {
        let from = INT::max(*range.start(), 0);
        let to = INT::max(*range.end(), from - 1);
        set_bits(value, from, to - from + 1, new_value)
    }
    /// Replace a portion of bits in the number with a new value.
    ///
    /// * If `start` < 0, position counts from the MSB (Most Significant Bit).
    /// * If `bits` ≤ 0, the number is not modified.
    /// * If `start` position + `bits` ≥ total number of bits, the bits after the `start` position are replaced.
    ///
    /// # Example
    ///
    /// ```rhai
    /// let x = 123456;
    ///
    /// x.set_bits(5, 8, 42);
    ///
    /// print(x);           // prints 124224
    ///
    /// x.set_bits(-16, 10, 42);
    ///
    /// print(x);           // prints 11821949021971776 on 64-bit
    /// ```
    #[rhai_fn(return_raw)]
    pub fn set_bits(value: &mut INT, bit: INT, bits: INT, new_value: INT) -> RhaiResultOf<()> {
        if bits <= 0 {
            return Ok(());
        }

        let bit = calc_index(INT_BITS, bit, true, || {
            ERR::ErrorBitFieldBounds(INT_BITS, bit, Position::NONE).into()
        })?;

        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        let bits = if bit + bits as usize > INT_BITS {
            INT_BITS - bit
        } else {
            bits as usize
        };

        if bit == 0 && bits == INT_BITS {
            *value = new_value;
            return Ok(());
        }

        // 2^bits - 1
        #[allow(clippy::cast_possible_truncation)]
        let mask = ((2 as UNSIGNED_INT).pow(bits as u32) - 1) as INT;

        *value &= !(mask << bit);
        *value |= (new_value & mask) << bit;

        Ok(())
    }
}
