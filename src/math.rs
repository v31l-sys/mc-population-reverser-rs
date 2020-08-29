//! This implements important math functions and central reverser object for cracking.
//!

use crate::lcg::*;

/// Structure defined for containing math types required for reversing population seeds.
pub struct PopulationReverser {
    pub mask_16: i64,
    pub mask_32: i64,
    pub mask_48: i64,
    pub mod_inverse: Vec<i32>,
    pub x_term: Vec<i32>,
    pub z_term: Vec<i32>,
}

lazy_static! {
    /// Of type PopulationReverser basic static utilities we need
    pub static ref POPREVERSER: PopulationReverser = PopulationReverser::new();
    pub static ref INTERNAL_MASK_32: i64 = population_reverser_math::external_mask(32);
}

impl Default for PopulationReverser {
    fn default() -> Self {
        Self {
            mask_16: 0,
            mask_32: 0,
            mask_48: 0,
            mod_inverse: Vec::with_capacity(65536),
            x_term: Vec::with_capacity(65536),
            z_term: Vec::with_capacity(65536),
        }
    }
}

pub mod bitwise_utils {
    /// Intended for implementing logical right shift (i.e. >>>)
    pub trait LogicalRightShift {
        fn lrs(&mut self, bits: u64);
    }

    impl LogicalRightShift for i64 {
        /// Logical Right Shift (i.e. >>>)
        ///
        /// ```
        /// fn lrs(&mut self, bits: u64) {
        ///     *self = (*self as u64 >> bits) as i64;
        /// }
        /// ```
        fn lrs(&mut self, bits: u64) {
            *self = (*self as u64 >> bits) as i64;
        }
    }
}

impl PopulationReverser {
    fn new() -> Self {
        let mut throwaway: PopulationReverser = PopulationReverser::default();

        throwaway.mask_16 = population_reverser_math::external_mask(16);
        throwaway.mask_32 = population_reverser_math::external_mask(32);
        throwaway.mask_48 = population_reverser_math::external_mask(48);

        for i in 0i32..population_reverser_math::pow2(16) as i32 {
            throwaway.mod_inverse.push(population_reverser_math::mod_inverse(i as i64, 16) as i32);

            throwaway.x_term.push(((FORWARD2.multiplier.wrapping_mul((i as i64 ^ FORWARD1.multiplier)
                & throwaway.mask_16).wrapping_add(FORWARD2.addend)) >> 16) as i32);

            throwaway.z_term.push(((FORWARD4.multiplier.wrapping_mul((i as i64 ^ FORWARD1.multiplier)
                & throwaway.mask_16).wrapping_add(FORWARD4.addend)) >> 16) as i32);
        }

        Self {
            ..throwaway
        }
    }
}

/// Math utils used in PopulationReverser
pub mod population_reverser_math {
    use std::num::Wrapping;

    pub const fn pow2(bits: i32) -> i64 {
        (1 as i64) << bits
    }

    pub fn mod_inverse(value: i64, k: i32) -> i64 {
        let value: Wrapping<i64> = Wrapping(value);
        let mut x: Wrapping<i64> = Wrapping(((((value.0 << 1) ^ value.0) & 4) << 1) ^ value.0);

        x += x - value * x * x;
        x += x - value * x * x;
        x += x - value * x * x;
        x += x - value * x * x;

        let return_masked = x.0 & external_mask(k);
        return_masked
    }

    /// ```
    /// if bits >= 64 { -1 } else { PopulationReverser::pow2(bits) - 1 }
    /// ```
    pub fn external_mask(bits: i32) -> i64 {
        if bits >= 64 { -1 } else { pow2(bits) - 1 }
    }

    /// ```
    /// value & external_mask(bits: i32)
    /// ```
    pub fn mask_calc(value: i64, bits: i32) -> i64 {
        value & external_mask(bits)
    }
}
