//! This is an abstraction of necessary JavaRandom functions.
//!

///JavaRandom struct with single parameter seed: i64
#[derive(Debug,Default, Copy, Clone)]
pub struct JavaRandom {
    seed: i64,
}

impl JavaRandom {
    ///equivalent to new JavaRandom(seed)
    /// ```
    /// seed: (seed ^ 0x5DEECE66D) & (((1 as i64) << 48) - 1)
    /// ```
    pub const fn get_random(seed: i64) -> Self {
        Self {
            seed: (seed ^ 0x5DEECE66D) & (((1 as i64) << 48) - 1),
        }
    }

    ///manually sets internal state of the seed
    pub fn set_seed_state(&mut self, seed: i64) {
        self.seed = seed;
    }

    ///gets current state of the internal seed
    pub fn get_seed_state(&self) -> i64 {
        self.seed
    }

    /// JavaRandom::nextLong
    /// ```
    /// pub fn next_long(&mut self) {
    ///     ((JavaRandom::random_next(&mut self, 32) as i64) << 32)
    ///         + JavaRandom::random_next(&mut self, 32)
    /// }
    /// ```
    pub fn next_long(&mut self) -> i64 {
        ((JavaRandom::random_next(self, 32) as i64) << 32)
            .wrapping_add(JavaRandom::random_next(self, 32) as i64)
    }

    /// JavaRandom::nextFloat
    /// ```
    /// pub fn next_float(&mut self) -> f32 {
    ///     (JavaRandom::random_next(&mut self, 24) as f32) / (1 << 24)
    /// }
    /// ```
    pub fn next_float(&mut self) -> f32 {
        (JavaRandom::random_next(self, 24) as f32) / (1 << 24) as f32
    }

    ///Fast Version of JavaRandom::nextInt
    ///probably fine to use for small bound sizes, assumes bound is positive obviously
    /// ```
    /// if (bound_size & -bound_size) == bound_size {
    ///             bound_size.wrapping_mul(JavaRandom::random_next(&mut self, 31)) >> 31
    /// }
    /// else {
    ///     JavaRandom::random_next(&mut self, 31) % bound_size
    /// }
    ///```
    pub fn next_int_fast(&mut self, bound: i32) -> i32 {
        if (bound & -bound) == bound {
            ((bound as i64).wrapping_mul(JavaRandom::random_next(self, 31) as i64) >> 31) as i32
        }
        else {
            JavaRandom::random_next(self, 31) % bound
        }
    }

    ///Slower Version of JavaRandom::nextInt
    ///proper implementation for larger bound sizes, assumes bound is positive obviously
    /// ```
    /// pub fn next_int(&mut self, bound_size: i32) -> i32 {
    ///     if (bound_size & -bound_size) == bound_size {
    ///         bound_size.wrapping_mul(JavaRandom::random_next(&mut self, 31)) >> 31
    ///     }
    ///     else {
    ///         let (mut bits, mut val): i32;
    ///         loop {
    ///             bits = JavaRandom::random_next(&mut self, 31);
    ///             val = bits % bound;
    ///             if bits - val + (bound - 1) >= 0 { break; }
    ///         }
    ///        val
    ///     }
    /// }
    /// ```
    pub fn next_int(&mut self, bound: i32) -> i32 {
        if (bound & -bound) == bound {
            ((bound as i64).wrapping_mul(JavaRandom::random_next(self, 31) as i64) >> 31) as i32
        }
        else {
            let mut bits: i32;
            let mut val: i32;
            loop {
                bits = JavaRandom::random_next(self, 31);
                val = bits % bound;
                if bits - val + (bound - 1) >= 0 { break; }
            }
            val
        }
    }

    /// Implementation of JavaRandom::next
    /// ```
    /// fn random_next(&mut self, bits: i32) -> i32 {
    ///     self.seed = self.seed.wrapping_mul(0x5DEECE66D).wrapping_add(0xB)
    ///         & (((1 as i64) << 48) - 1);
    ///
    ///     (self.seed >> (48 - bits)) as i32
    /// }
    /// ```
    fn random_next(&mut self, bits: i32) -> i32 {
        self.seed = self.seed.wrapping_mul(0x5DEECE66D).wrapping_add(0xB)
            & (((1 as i64) << 48) - 1);

        (self.seed >> (48 - bits)) as i32
    }
}
