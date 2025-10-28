use num_bigint::{BigInt, Sign};

use crate::{Term, unit::Unit};

#[derive(Debug)]
pub struct Lft {
    pub mat: [BigInt; 4],
    pub egest_enabled: bool,
}

impl Lft {
    pub fn identity() -> Self {
        Self {
            mat: [1.into(), 0.into(), 0.into(), 1.into()],
            egest_enabled: true,
        }
    }

    // all the methods of the form is_*
    // require BOTH endpoints to be *
    // an LFT can be neither zero, nor positive, nor negative

    pub fn is_undefined(&self) -> bool {
        self.mat.iter().all(|i| *i == 0.into())
    }

    pub fn is_inf(&self) -> bool {
        self.mat[2] == 0.into()
            && self.mat[3] == 0.into()
            && !(self.mat[0] == 0.into() && self.mat[1] == 0.into())
    }

    pub fn is_zero(&self) -> bool {
        self.mat[0] == 0.into()
            && self.mat[1] == 0.into()
            && !(self.mat[2] == 0.into() && self.mat[3] == 0.into())
    }

    pub fn is_nonnegative(&self) -> bool {
        !((self.mat[0].sign() == Sign::Minus) ^ (self.mat[2].sign() == Sign::Minus))
            && !(((self.mat[0].clone() + self.mat[1].clone()).sign() == Sign::Minus)
                ^ ((self.mat[2].clone() + self.mat[3].clone()).sign() == Sign::Minus))
    }

    pub fn is_nonpositive(&self) -> bool {
        ((self.mat[0].sign() == Sign::Minus) ^ (self.mat[2].sign() == Sign::Minus)
            || self.mat[0] == 0.into())
            && (((self.mat[0].clone() + self.mat[1].clone()).sign() == Sign::Minus)
                ^ ((self.mat[2].clone() + self.mat[3].clone()).sign() == Sign::Minus)
                || self.mat[0] == 0.into() && self.mat[1] == 0.into())
    }

    pub fn is_positive(&self) -> bool {
        self.mat[0] != 0.into()
            && self.mat[0].clone() + self.mat[1].clone() != 0.into()
            && !((self.mat[0].sign() == Sign::Minus) ^ (self.mat[2].sign() == Sign::Minus))
            && !(((self.mat[0].clone() + self.mat[1].clone()).sign() == Sign::Minus)
                ^ ((self.mat[2].clone() + self.mat[3].clone()).sign() == Sign::Minus))
    }

    pub fn is_negative(&self) -> bool {
        self.mat[0] != 0.into()
            && self.mat[0].clone() + self.mat[1].clone() != 0.into()
            && (self.mat[0].sign() == Sign::Minus) ^ (self.mat[2].sign() == Sign::Minus)
            && ((self.mat[0].clone() + self.mat[1].clone()).sign() == Sign::Minus)
                ^ ((self.mat[2].clone() + self.mat[3].clone()).sign() == Sign::Minus)
    }

    pub fn intervals(&self) -> ((BigInt, BigInt), (BigInt, BigInt)) {
        // (just removes the trailing zeros)
        // (without which the numbers should be in fully reduced form)
        fn prettify(mut a: BigInt, mut b: BigInt) -> (BigInt, BigInt) {
            let g = a
                .trailing_zeros()
                .unwrap_or(0)
                .min(b.trailing_zeros().unwrap_or(0));
            if a < 0.into() && b < 0.into() {
                a = -a;
                b = -b;
            }
            // if a == 0.into() {
            //     b = 1.into();
            // }
            // if b == 0.into() {
            //     a = 1.into();
            // }
            (a >> g, b >> g)
        }
        (
            prettify(self.mat[0].clone(), self.mat[2].clone()),
            prettify(
                self.mat[0].clone() + self.mat[1].clone(),
                self.mat[2].clone() + self.mat[3].clone(),
            ),
        )
    }

    // rounds towards 0
    // returns the first term of this lft's continued fraction
    // (if x=oo and x=1 agree)
    pub fn trunc(&self) -> Option<BigInt> {
        let ((n1, d1), (n2, d2)) = self.intervals();
        let c1 = n1.checked_div(&d1)?;
        let c2 = n2.checked_div(&d2)?;
        (c1 == c2).then_some(c1)
    }

    // rounds towards -oo
    // returns t if we're both in [t, t+1) for int t
    pub fn floor(&self) -> Option<BigInt> {
        let ((n1, d1), (n2, d2)) = self.intervals();
        let i1 = n1.checked_div(&d1)?;
        let i2 = n2.checked_div(&d2)?;
        let a1 = n1 == i1.clone() * d1.clone();
        let a2 = n2 == i2.clone() * d2.clone();
        let f1 = if a1 || self.is_positive() { i1 } else { i1 - 1 };
        let f2 = if a2 || self.is_positive() { i2 } else { i2 - 1 };
        (f1 == f2).then_some(f1)
    }

    // rounds towards +oo
    // returns t if we're both in (t-1, t] for int t
    pub fn ceil(&self) -> Option<BigInt> {
        let ((n1, d1), (n2, d2)) = self.intervals();
        let i1 = n1.checked_div(&d1)?;
        let i2 = n2.checked_div(&d2)?;
        let a1 = n1 == i1.clone() * d1.clone();
        let a2 = n2 == i2.clone() * d2.clone();
        let f1 = if a1 || self.is_negative() { i1 } else { i1 + 1 };
        let f2 = if a2 || self.is_negative() { i2 } else { i2 + 1 };
        (f1 == f2).then_some(f1)
    }

    // rounds half to even
    // returns t if we're both in [t, t+1/2)
    // returns t+1 if we're both in (t+1/2, t+1)
    // if we're at t+1/2, returns t if it's even, t+1 if it's not
    // for int t
    pub fn round(&self) -> Option<BigInt> {
        let ((n1, d1), (n2, d2)) = self.intervals();
        let i1 = n1.checked_div(&d1)?;
        let i2 = n2.checked_div(&d2)?;
        let h1 = (n1.clone() - i1.clone() * d1.clone()).into_parts().1;
        let h2 = (n2.clone() - i2.clone() * d2.clone()).into_parts().1;
        let offset = if self.is_positive() { 1 } else { -1 };
        if i1 != i2 {
            return None;
        }
        let (d1, d2) = (d1.into_parts().1, d2.into_parts().1);
        if h1.clone() * 2u32 < d1 && h2.clone() * 2u32 < d2 {
            Some(i1)
        } else if h1.clone() * 2u32 > d1 && h2.clone() * 2u32 > d2 {
            Some(i1 + offset)
        } else if h1.clone() * 2u32 == d1 && h2.clone() * 2u32 == d2 {
            Some(if !i1.bit(0) { i1 } else { i1 + offset })
        } else {
            None
        }
    }
}

impl Unit for Lft {
    fn ingest_x(&mut self, x: Term) {
        match x {
            Term::Empty => {}
            Term::Ord => {
                if !self.mat[1].bit(0) && !self.mat[3].bit(0) {
                    self.mat[1] >>= 1;
                    self.mat[3] >>= 1;
                } else {
                    self.mat[0] <<= 1;
                    self.mat[2] <<= 1;
                }
            }
            Term::DRec => {
                self.mat.swap(0, 1);
                self.mat.swap(2, 3);
                self.mat[0] += self.mat[1].clone();
                self.mat[2] += self.mat[3].clone();
            }
            Term::Rec => {
                self.mat.swap(0, 1);
                self.mat.swap(2, 3);
            }
            Term::Neg => {
                self.mat[0] = -self.mat[0].clone();
                self.mat[2] = -self.mat[2].clone();
            }
            Term::Inf => {
                self.mat[1] = self.mat[0].clone();
                self.mat[3] = self.mat[2].clone();
            }
            Term::Undefined => {
                self.mat = [0.into(), 0.into(), 0.into(), 0.into()];
            }
        }
    }

    fn ingest_y(&mut self, _y: Term) {
        // (LFTs don't have a y in their expression)
    }

    fn egest_z(&mut self) -> Term {
        if !self.egest_enabled {
            return Term::Empty;
        }

        if self.is_undefined() {
            return Term::Undefined;
        }
        if self.is_inf() {
            // we like distinguishing between -oo and +oo
            if self.mat[0] < 0.into() || self.mat[1] < 0.into() {
                self.mat[0] = -self.mat[0].clone();
                self.mat[1] = -self.mat[1].clone();
                return Term::Neg;
            }
            return Term::Inf;
        }

        // n0 is x=oo, n1 is x=1
        let mut n0 = self.mat[0].clone();
        let mut n1 = self.mat[0].clone() + self.mat[1].clone();
        let mut d0 = self.mat[2].clone();
        let mut d1 = self.mat[2].clone() + self.mat[3].clone();
        let _ = crate::rationalize(&mut n0, &mut d0);
        let _ = crate::rationalize(&mut n1, &mut d1);
        let nums_agreed = (n0 < 0.into()) == (n1 < 0.into());
        let dens_agreed = (d0 < 0.into()) == (d1 < 0.into());
        let den_zeros = d0 == 0.into() || d1 == 0.into();
        let well_defined =
            nums_agreed && dens_agreed && (!den_zeros || crate::unit::arith::MORE_AGGRESSIVE);

        if well_defined {
            if n0.sign() == Sign::Minus && n1.sign() == Sign::Minus {
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
        }

        Term::Empty
    }
}
