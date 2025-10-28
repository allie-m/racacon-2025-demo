use std::fmt::Debug;

use num_bigint::BigInt;

use crate::{Term, unit::Unit};

pub mod consts {
    use crate::unit::cfrac::FromCFrac;

    pub fn e() -> FromCFrac {
        FromCFrac {
            iter: {
                let mut i = -1;
                Box::new(std::iter::from_fn(move || {
                    i += 1;
                    if i == 0 {
                        Some((2, 1, 1, 1))
                    } else if i % 3 == 2 {
                        Some((2 + 2 * (i / 3), 1, 1, 1))
                    } else {
                        Some((1, 1, 1, 1))
                    }
                }))
            },
            mat: [1.into(), 0.into(), 0.into(), 1.into()],
        }
    }

    pub fn pi() -> FromCFrac {
        FromCFrac {
            iter: {
                let mut i = 0;
                Box::new(std::iter::from_fn(move || {
                    i += 1;
                    Some((i * 2 - 1, 1, i * i, 1))
                }))
            },
            mat: [0.into(), 4.into(), 1.into(), 0.into()],
        }
    }
}

// generalized cfrac input gives us a lot of nice things
// in particular, we can use this to easily get nice things like e^q for q in Q
// unfortunately, if we want to get something like e^x for continued logarithm x, we need a different method
pub struct FromCFrac {
    // each (p/q, r/s) replaces x with p/q + (r/s)/x
    // we love us some generalized continued fractions
    pub iter: Box<dyn Iterator<Item = (i64, i64, i64, i64)>>,
    pub mat: [BigInt; 4],
}

impl Debug for FromCFrac {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FromCFrac: (internal iter cannot be inspected); mat: {:?}",
            self.mat
        )
    }
}

impl Unit for FromCFrac {
    // we don't ingest clog terms
    // we do ingest from iter
    // but that's once per egest
    fn ingest_x(&mut self, _x: Term) {}
    fn ingest_y(&mut self, _y: Term) {}

    fn egest_z(&mut self) -> Term {
        let terms = self.iter.next();
        // ingest from iter
        match terms {
            Some((p, q, r, s)) => {
                let i = self.mat[0].clone();
                self.mat[0] *= p * s;
                self.mat[0] += self.mat[1].clone() * q * s;
                self.mat[1] = i * r * q;
                let i = self.mat[2].clone();
                self.mat[2] *= p * s;
                self.mat[2] += self.mat[3].clone() * q * s;
                self.mat[3] = i * r * q;
            }
            None => {
                // ingesting oo
                self.mat[1] = self.mat[0].clone();
                self.mat[3] = self.mat[2].clone();
            }
        }
        // // ugh if the cfrac is generalized rather than simple
        // // the gcd grows, and can get annoyingly big
        // // like up to half the bits might be redundant
        // // but i can't do anything about it because calculating the gcd scales linearly with the number of bits
        // // and it's not worth it to do like 100 divisions per cycle
        // {
        //     fn gcd(mut a: BigInt, mut b: BigInt) -> BigInt {
        //         let mut i = 0;
        //         while b != 0.into() {
        //             i += 1;
        //             let t = b.clone();
        //             b = a % b;
        //             a = t;
        //         }
        //         println!("{} iters to find the gcd", i);
        //         a
        //     }
        //     let g = gcd(
        //         self.mat[0].clone(),
        //         gcd(
        //             self.mat[1].clone(),
        //             gcd(self.mat[2].clone(), self.mat[3].clone()),
        //         ),
        //     );
        //     self.mat[0] /= g.clone();
        //     self.mat[1] /= g.clone();
        //     self.mat[2] /= g.clone();
        //     self.mat[3] /= g.clone();
        //     println!("{:?} | {:?}", self.mat, g);
        // }

        // nowww we egest a term
        let n0 = self.mat[0].clone();
        let n1 = self.mat[0].clone() + self.mat[1].clone();
        let d0 = self.mat[2].clone();
        let d1 = self.mat[2].clone() + self.mat[3].clone();
        if n0 == 0.into() && n1 == 0.into() && d0 == 0.into() && d1 == 0.into() {
            return Term::Undefined;
        }
        if d0 == 0.into() && d1 == 0.into() {
            return Term::Inf;
        }
        if (self.mat[0] < 0.into()) != (self.mat[2] < 0.into()) {
            self.mat[0] = -self.mat[0].clone();
            self.mat[1] = -self.mat[1].clone();
            return Term::Neg;
        } else if (n0.clone() >> 1) >= d0 && (n1.clone() >> 1) >= d1 {
            if !self.mat[0].bit(0) && !self.mat[1].bit(0) {
                self.mat[0] >>= 1;
                self.mat[1] >>= 1;
            } else {
                self.mat[2] <<= 1;
                self.mat[3] <<= 1;
            }
            return Term::Ord;
        } else if n0 >= d0 && n1 >= d1 && (n0.clone() >> 1) < d0 && (n1.clone() >> 1) < d1 {
            self.mat[0] -= self.mat[2].clone();
            self.mat[1] -= self.mat[3].clone();
            self.mat.swap(0, 2);
            self.mat.swap(1, 3);
            return Term::DRec;
        } else if n0 < d0 && n1 < d1 {
            self.mat.swap(0, 2);
            self.mat.swap(1, 3);
            return Term::Rec;
        }
        Term::Empty
    }
}
