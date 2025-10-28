use std::fmt::Debug;

pub mod unit;
pub mod workgroup;

// basic idea:
// - (use bigints)
// - Unit (arith, sqrt, compare, cfrac/rat/float conversions; they ingest and post their egestions)
// - Workgroup (dynamically extensible/composable DAG; exp, log, etc)
// stuff that only has 2 stages we meter to 3
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Term {
    #[default]
    Empty,
    Ord,
    DRec,
    Rec,
    Neg,
    Inf,
    Undefined,
}

impl Debug for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "ø"),
            Self::Ord => write!(f, "1"),
            Self::DRec => write!(f, "0"),
            Self::Rec => write!(f, "/"),
            Self::Neg => write!(f, "-"),
            Self::Inf => write!(f, "∞"),
            Self::Undefined => write!(f, "!"),
        }
    }
}

// calculates the sign of the nums
// flips the signs of both of them if the denominator is negative
// that way, we're guaranteed to have one of +/+, -/+, 0/+, or whatever/0
// we have -oo but no -0
pub(crate) fn rationalize(
    num: &mut num_bigint::BigInt,
    den: &mut num_bigint::BigInt,
) -> num_bigint::Sign {
    let (n, d) = (num.sign(), den.sign());
    if d == num_bigint::Sign::Minus {
        *num *= -1;
        *den *= -1;
    }
    if n == num_bigint::Sign::NoSign {
        num_bigint::Sign::NoSign
    } else if (n == num_bigint::Sign::Minus) ^ (d == num_bigint::Sign::Minus) {
        num_bigint::Sign::Minus
    } else {
        num_bigint::Sign::Plus
    }
}
