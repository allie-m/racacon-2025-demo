// use num_bigint::{BigInt, BigUint};

// use crate::{Term, unit::Unit};

// // TODO: delete this?

// pub struct FromRational {
//     pub num: BigUint,
//     pub den: BigUint,
//     pub sign: i8,
// }

// impl Unit for FromRational {
//     fn ingest_x(&mut self, _x: Term) {}
//     fn ingest_y(&mut self, _y: Term) {}
//     fn egest_z(&mut self) -> Term {
//         if self.den == 0u32.into() {
//             Term::Inf
//         } else if self.sign < 0 {
//             self.sign = 1;
//             Term::Neg
//         } else if (self.num.clone() >> 1) >= self.den {
//             if !self.num.bit(0) {
//                 self.num >>= 1;
//             } else {
//                 self.den <<= 1;
//             }
//             Term::Ord
//         } else if self.num >= self.den {
//             self.num -= self.den.clone();
//             std::mem::swap(&mut self.num, &mut self.den);
//             Term::DRec
//         } else {
//             std::mem::swap(&mut self.num, &mut self.den);
//             Term::Rec
//         }
//     }
// }
