//! These are the functions used for reversing a population seed to a world seed(s)
//!

/// Utility module for performing population seed -> world seed, takes a couple of milliseconds or less.
pub mod population_seed_utils {
    use crate::lcg::*;
    use std::collections::HashSet;
    use random::chunk_random::*;
    use crate::math::*;

    fn lift(value: i64, bit: i32, target: i64, bits: i32, offset: i32, chunk_random: &mut ChunkRandom,
            x: i32, z: i32, world_seeds: &mut Vec<i64>)
    {
        let calculated: i64 = chunk_random.set_population_seed(value, x, z);
        if bit >= bits {
            if population_reverser_math::mask_calc(target, bit + offset) == population_reverser_math::mask_calc(calculated, bit + offset)
            {
                world_seeds.push(value);
            } else if population_reverser_math::mask_calc(target, bit) == population_reverser_math::mask_calc(calculated, bit + offset) {
                lift(value, bit + 1, target, bits, offset, chunk_random, x, z, world_seeds);
                lift(value | population_reverser_math::pow2(bit + offset),
                     bit + 1, target, bits, offset, chunk_random, x, z, world_seeds);
            }
        }
    }

    /// This function will find world seeds corresponding with the population seed and chunk section coordinates.
    pub fn reverse(population_seed: i64, x: i32, z: i32) -> Vec<i64> {
        let mut world_seeds: Vec<i64> = Vec::default();
        let mut chunk_random: ChunkRandom = ChunkRandom::default();

        let mut c: i64; //a is the upper 16 bits, b >> 16, c << 32
        let e: i64 = population_seed & POPREVERSER.mask_32;
        let f: i64 = population_seed & POPREVERSER.mask_16;

        let mut free_bits: i32 = ((x | z) as i64).trailing_zeros() as i32;
        c = population_reverser_math::mask_calc(population_seed, free_bits);
        c |= if free_bits == 64 { 0 } else {
            ((x ^ z) as i64 ^ population_seed)
                & population_reverser_math::pow2(free_bits)
        };
        free_bits += 1;

        let increment: i32 = if free_bits >= 64 { 1 } else { population_reverser_math::pow2(free_bits) as i32 };

        let first_mult: i64 = (FORWARD2.multiplier.wrapping_mul(x as i64)
            .wrapping_add(FORWARD4.multiplier.wrapping_mul(z as i64))) & POPREVERSER.mask_16;

        let mult_trailing_zeroes: i32 = first_mult.trailing_zeros() as i32;

        if mult_trailing_zeroes >= 16 {
            if free_bits >= 16 {
                lift(c, free_bits - 16, population_seed, 32, 16,
                     &mut chunk_random, x, z, &mut world_seeds);
            } else {
                loop {
                    if c >= 1 << 16 { break; }

                    lift(c, 0, population_seed, 32, 16,
                         &mut chunk_random, x, z, &mut world_seeds);

                    c += increment as i64;
                }
            }

            return world_seeds
        }

        let first_mult_inv: i64 = POPREVERSER.mod_inverse[(first_mult >> mult_trailing_zeroes) as usize] as i64;
        let offsets: HashSet<i32> = get_offsets(x, z);

        loop {
            if c >= 1 << 16 { break; }

            let target: i64 = (c ^ f) & POPREVERSER.mask_16;
            let magic: i64 = (x.wrapping_mul(POPREVERSER.x_term[c as usize]))
                .wrapping_add(z.wrapping_mul(POPREVERSER.z_term[c as usize])) as i64;

            for offset in &offsets {
                add_world_seeds(target - ((magic + *offset as i64) & POPREVERSER.mask_16),
                                mult_trailing_zeroes, first_mult_inv, c, e, x, z, population_seed, &mut chunk_random, &mut world_seeds);
            }

            c += increment as i64;
        }

        return world_seeds
    }

    fn add_world_seeds(first_addend: i64, mult_trailing_zeroes: i32, first_mult_inv: i64, c: i64, e: i64, x: i32, z: i32,
                       population_seed: i64, chunk_random: &mut ChunkRandom, world_seeds: &mut Vec<i64>)
    {
        if first_addend.trailing_zeros() < mult_trailing_zeroes as u32 { return }

        let mask: i64 = population_reverser_math::external_mask(16 - mult_trailing_zeroes);
        let increment: i64 = population_reverser_math::pow2(16 - mult_trailing_zeroes);

        let mut indexer: i64 = ((first_mult_inv.wrapping_mul(first_addend) >> mult_trailing_zeroes)
            ^ (FORWARD1.multiplier >> 16)) & mask;

        //could these have been while loops? yes, do I care? nah
        loop {
            if indexer >= 1 << 16 { break; }

            let k: i64 = (indexer << 16) + c;
            let target2: i64 = (k ^ e) >> 16;
            let second_addend: i64 = get_partial_addend(k, x, z) & POPREVERSER.mask_16;

            if (target2 - second_addend).trailing_zeros() < mult_trailing_zeroes as u32 {
                indexer += increment;
                continue;
            }

            let mut a: i64 = ((first_mult_inv.wrapping_mul(target2 - second_addend)
                >> mult_trailing_zeroes) ^ (FORWARD1.multiplier >> 32)) & mask;

            loop {
                if a >= 1 << 16 { break; }

                if chunk_random.set_population_seed((a << 32) + k, x, z)
                    != population_seed {
                    a += increment;
                    continue;
                }

                world_seeds.push((a << 32) + k);

                a += increment;
            }

            indexer += increment;
        }
    }

    fn get_offsets(x: i32, z: i32) -> HashSet<i32> {
        let mut offsets: HashSet<i32> = Default::default();

        for i in 0..2 {
            for j in 0..2 {
                offsets.insert(x * i + z * j);
            }
        }

        offsets
    }

    fn get_partial_addend(partial_seed: i64, x: i32, z: i32) -> i64 {
        let a: i64 = ((((FORWARD2.multiplier.wrapping_mul((partial_seed ^ FORWARD1.multiplier) & (*INTERNAL_MASK_32))
            .wrapping_add(FORWARD2.addend)) & POPREVERSER.mask_48) >> 16) as i32) as i64;

        let b: i64 = ((((FORWARD4.multiplier.wrapping_mul((partial_seed ^ FORWARD1.multiplier) & (*INTERNAL_MASK_32))
            .wrapping_add(FORWARD4.addend)) & POPREVERSER.mask_48) >> 16) as i32) as i64;

        return (x as i64).wrapping_mul(a | 1).wrapping_add((z as i64).wrapping_mul(b | 1)) >> 16;
    }
}
