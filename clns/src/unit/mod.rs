// - arith (brabec's algo)
// - sqrt (my algo)
// - compare
// - cfrac
//    - consts
//    - conversion to/from
// - rational interval conversion
//    - TODO investigate faster converging approximations
// - integer ops
//    - TODO: investigate these
// - exp/log/trig (these should probably be workgroups)

use std::fmt::Debug;

use crate::Term;

pub mod arith;
pub mod cfrac;
pub mod cmp;
pub mod int;
pub mod lft;
pub mod rational;
pub mod sqrt;

use arith::Arith;
use cfrac::FromCFrac;
use cmp::Compare;
use int::Modulo;
use lft::Lft;
use sqrt::Sqrt;

use super::workgroup::exp::ExpTaylor;
use super::workgroup::log2::Log2;

#[enum_dispatch::enum_dispatch]
#[derive(Debug)]
pub enum UnitUnion {
    Arith,
    CLogs,
    Compare,
    FromCFrac,
    Lft,
    Modulo,
    Sqrt,
    ExpTaylor,
    Log2,
}

#[enum_dispatch::enum_dispatch(UnitUnion)]
pub trait Unit {
    fn ingest_x(&mut self, x: Term);
    fn ingest_y(&mut self, y: Term);
    fn egest_z(&mut self) -> Term;
}

// you can put anything that'll generate a clog stream into here
// unload a vector, a recurring sequence, whatever
pub struct CLogs {
    pub terms: Box<dyn FnMut() -> Term>,
}

impl Debug for CLogs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CLogs: (internal iter cannot be inspected)")
    }
}

impl Unit for CLogs {
    fn ingest_x(&mut self, _: Term) {}
    fn ingest_y(&mut self, _: Term) {}
    fn egest_z(&mut self) -> Term {
        (self.terms)()
    }
}
