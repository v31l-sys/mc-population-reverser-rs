//! This is an abstraction of necessary ChunkRandom functions and their reverse.
//!

use crate::lcg::*;
use crate::java_random::*;

///ChunkRandom struct that implements JavaRandom
#[derive(Debug, Default, Copy, Clone)]
pub struct ChunkRandom {
    pub rand: JavaRandom,
}

impl ChunkRandom {
    ///equivalent to new JavaRandom(seed)
    /// ```
    /// seed: (seed ^ 0x5DEECE66D) & (((1 as i64) << 48) - 1)
    /// ```
    pub fn get_random(&mut self, seed: i64) { self.rand = JavaRandom::get_random(seed); }

    ///manually sets internal state of the seed
    pub fn set_seed_state(&mut self, seed: i64) { self.rand.set_seed_state(seed); }

    ///gets current state of the internal seed
    pub fn get_seed_state(&self) -> i64 {
        self.rand.get_seed_state()
    }

    ///Implementation of ChunkRandom population seed setting that allows for generating population seeds
    /// ```
    /// pub fn set_population_seed(&mut self, world_seed: i64, x: i32, z: i32) -> i64 {
    ///     self.rand = JavaRandom::get_random(world_seed);
    ///
    ///     let s1: i64 = self.rand.next_long() | 0x1;
    ///     let s2: i64 = self.rand.next_long() | 0x1;
    ///
    ///     let new_seed: i64 = s1.wrapping_mul(x as i64).wrapping_add(s2.wrapping_mul(z as i64)) ^ world_seed;
    ///
    ///     self.rand = JavaRandom::get_random(new_seed);
    ///     new_seed & (((1 as i64) << 48) - 1)
    /// }
    /// ```
    pub fn set_population_seed(&mut self, world_seed: i64, x: i32, z: i32) -> i64 {
        self.rand = JavaRandom::get_random(world_seed);

        let s1: i64 = self.rand.next_long() | 0x1;
        let s2: i64 = self.rand.next_long() | 0x1;

        let new_seed: i64 = s1.wrapping_mul(x as i64).wrapping_add(s2.wrapping_mul(z as i64)) ^ world_seed;

        self.rand = JavaRandom::get_random(new_seed);
        new_seed
    }

    ///Implementation of ChunkRandom decorator seed setting that allows for generating decorator seeds
    /// ```
    /// pub fn set_decorator_seed(&mut self, seed: i64, index: i16, step: i16) -> i64 {
    ///    let new_seed = seed + index as i64 + 10000 * step as i64;
    ///    self.rand = JavaRandom::get_random(new_seed);
    ///    new_seed
    /// }
    /// ```
    pub fn set_decorator_seed(&mut self, seed: i64, index: i16, step: i16) -> i64 {
        let new_seed = seed + index as i64 + 10000 * step as i64;
        self.rand = JavaRandom::get_random(new_seed);
        new_seed
    }

    ///Implementation of JavaRandom::next that allows for the use of arbitrary LCG
    /// ```
    /// fn random_next(&mut self, bits: i32, lcg: &LCG) -> i32 {
    ///     match lcg.lcg_type {
    ///         LCGType::FORWARD => {
    ///             self.rand.set_seed_state((self.rand.get_seed_state().wrapping_mul(lcg.multiplier)
    ///                 .wrapping_add(lcg.addend) & lcg.modulus));
    ///
    ///             (self.rand.get_seed_state() >> (48 - bits)) as i32
    ///         },
    ///         LCGType::REVERSE => {
    ///             (((self.rand.get_seed_state() - lcg.addend).wrapping_mul(lcg.multiplier)
    ///                 & lcg.modulus) >> (48 - bits)) as i32
    ///         },
    ///     }
    /// }
    /// ```
    pub fn random_next(&mut self, bits: i32, lcg: &LCG) -> i32 {
        match lcg.lcg_type {
            LCGType::FORWARD => {
                self.rand.set_seed_state(self.rand.get_seed_state().wrapping_mul(lcg.multiplier)
                    .wrapping_add(lcg.addend) & lcg.modulus);

                (self.rand.get_seed_state() >> (48 - bits)) as i32
            },
            LCGType::REVERSE => {
                self.rand.set_seed_state((self.rand.get_seed_state().wrapping_sub(lcg.addend))
                    .wrapping_mul(lcg.multiplier) & lcg.modulus);

                (self.get_seed_state() >> (48 - bits)) as i32
            },
        }
    }
}
