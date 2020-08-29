//! This is an abstraction of LCG and contains combination generation.
//!

// NOTE: this class can be extended to handle multiple types of LCGs but idc about that

use std::num::Wrapping;
use crate::math::bitwise_utils::LogicalRightShift;

#[derive(Debug, Copy, Clone)]
pub enum LCGType {
    FORWARD,
    REVERSE,
}

/// Arbitrary LCG of any combination
#[derive(Debug, Copy, Clone)]
pub struct LCG {
    pub multiplier: i64,
    pub addend: i64,
    pub modulus: i64,
    pub lcg_type: LCGType,
}

const JAVA_MULTIPLIER: i64 = 0x5DEECE66D;
const JAVA_MODULUS: i64 = ((1 as i64) << 48) - 1;
const JAVA_ADDEND: i64 = 0xB;

lazy_static! {
    ///Equivalent of the default Java Random LCG
    pub static ref FORWARD1: LCG = LCG::generate(1, JAVA_MULTIPLIER, LCGType::FORWARD);
    //
    pub static ref FORWARD2: LCG = LCG::generate(2, JAVA_MULTIPLIER, LCGType::FORWARD);
    pub static ref FORWARD3: LCG = LCG::generate(3, JAVA_MULTIPLIER, LCGType::FORWARD);
    pub static ref FORWARD4: LCG = LCG::generate(4, JAVA_MULTIPLIER, LCGType::FORWARD);

    ///Equivalent of the default Java Random LCG Inverse
    pub static ref BACKWARD1: LCG = LCG::generate(1, JAVA_MULTIPLIER, LCGType::REVERSE);
    //
    pub static ref BACKWARD2: LCG = LCG::generate(2, JAVA_MULTIPLIER, LCGType::REVERSE);
    pub static ref BACKWARD3: LCG = LCG::generate(3, JAVA_MULTIPLIER, LCGType::REVERSE);
    pub static ref BACKWARD4: LCG = LCG::generate(4, JAVA_MULTIPLIER, LCGType::REVERSE);
    pub static ref BACKWARD5: LCG = LCG::generate(5, JAVA_MULTIPLIER, LCGType::REVERSE);
    pub static ref BACKWARD6: LCG = LCG::generate(6, JAVA_MULTIPLIER, LCGType::REVERSE);
}

impl LCG {
    /// Calculates the combination of multiple calls to LCG->next for efficiency
    /// This is an implementation set up for JavaRandom but it can be extended.<br><br>
    ///
    /// X[n+1] = (aX[n] + c) mod m<br><br>
    ///
    /// # Arguments
    ///
    /// * `iterations` - An arbitrary amount of iterations of the LCG to combine up to 2^16 - 1.
    /// * `inverse` - A bool that specifies whether you want to calculate using the inverse of the LCG or not.
    ///
    pub fn generate(mut iterations: i64, base_multiplier: i64, lcg_type: LCGType) -> Self {
        let reverse = if iterations > 0 { false } else { true };

        let mut multiplier = Wrapping(1 as i64);
        let mut addend = Wrapping(0 as i64);

        let mut intermediate_multiplier = Wrapping(base_multiplier);
        let mut intermediate_addend = Wrapping(JAVA_ADDEND);

        while iterations != 0 {
            if (iterations & 1) != 0 {
                multiplier *= intermediate_multiplier;
                addend = intermediate_multiplier * addend + intermediate_addend;
            }

            intermediate_addend = (intermediate_multiplier + Wrapping(1 as i64)) * intermediate_addend;
            intermediate_multiplier *= intermediate_multiplier;

            iterations.lrs(1);
        }

        let multiplier = multiplier.0 & JAVA_MODULUS;
        let addend = addend.0 & JAVA_MODULUS;

        //If the function is the reverse.. calculate the inverse (basically)
        match lcg_type {
            LCGType::FORWARD => {
                Self {
                    multiplier,
                    addend,
                    modulus: JAVA_MODULUS,
                    lcg_type: if reverse == false { LCGType::FORWARD } else { LCGType::REVERSE },
                }
            },

            LCGType::REVERSE => {
                //essentially... -1 gets the inverse using this function
                //the return of that generate returns a garbage addend value
                //so that has to be replaced with the addend already calculated
                Self {
                    addend,
                    ..LCG::generate(-1, multiplier, LCGType::FORWARD)
                }
            },
        }
    }
}
