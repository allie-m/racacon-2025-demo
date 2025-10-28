use num_bigint::{BigInt, Sign};

use crate::{Term, unit::Unit};

// follows brabec's algorithm exactly if set to false
// otherwise, allows egestion of 1s w/infinite endpoints
// also basically removes the singularity flag
// we can emit speculative / and singularity 1 whenever
// cause retracted inputs might cause us to enter the singularity
// without tripping the flag
pub const MORE_AGGRESSIVE: bool = true;

#[derive(Debug)]
pub struct Arith {
    pub mat: [BigInt; 8],
    singularity: bool,
}

impl Arith {
    pub fn create(mat: [BigInt; 8]) -> Self {
        Self {
            mat,
            singularity: false,
        }
    }
}

// manual egests; this way we can do feedbacks if we have to
impl Arith {
    pub(crate) fn egest(&mut self, term: Term) {
        match term {
            Term::Empty | Term::Inf => {}
            Term::Ord => {
                if !self.mat[0].bit(0)
                    && !self.mat[1].bit(0)
                    && !self.mat[2].bit(0)
                    && !self.mat[3].bit(0)
                {
                    self.mat[0] >>= 1;
                    self.mat[1] >>= 1;
                    self.mat[2] >>= 1;
                    self.mat[3] >>= 1;
                } else {
                    self.mat[4] <<= 1;
                    self.mat[5] <<= 1;
                    self.mat[6] <<= 1;
                    self.mat[7] <<= 1;
                }
            }
            Term::DRec => {
                self.mat[0] -= self.mat[4].clone();
                self.mat[1] -= self.mat[5].clone();
                self.mat[2] -= self.mat[6].clone();
                self.mat[3] -= self.mat[7].clone();
                self.mat.swap(0, 4);
                self.mat.swap(1, 5);
                self.mat.swap(2, 6);
                self.mat.swap(3, 7);
            }
            Term::Rec => {
                self.mat.swap(0, 4);
                self.mat.swap(1, 5);
                self.mat.swap(2, 6);
                self.mat.swap(3, 7);
            }
            Term::Neg => {
                self.mat[0] = -self.mat[0].clone();
                self.mat[1] = -self.mat[1].clone();
                self.mat[2] = -self.mat[2].clone();
                self.mat[3] = -self.mat[3].clone();
            }
            Term::Undefined => {
                self.mat = [
                    0.into(),
                    0.into(),
                    0.into(),
                    0.into(),
                    0.into(),
                    0.into(),
                    0.into(),
                    0.into(),
                ]
            }
        }
    }
}

impl Unit for Arith {
    fn ingest_x(&mut self, x: Term) {
        match x {
            Term::Empty => {}
            Term::Ord => {
                if !self.mat[2].bit(0)
                    && !self.mat[3].bit(0)
                    && !self.mat[6].bit(0)
                    && !self.mat[7].bit(0)
                {
                    self.mat[2] >>= 1;
                    self.mat[3] >>= 1;
                    self.mat[6] >>= 1;
                    self.mat[7] >>= 1;
                } else {
                    self.mat[0] <<= 1;
                    self.mat[1] <<= 1;
                    self.mat[4] <<= 1;
                    self.mat[5] <<= 1;
                }
            }
            Term::DRec => {
                self.mat.swap(0, 2);
                self.mat.swap(1, 3);
                self.mat.swap(4, 6);
                self.mat.swap(5, 7);
                self.mat[0] += self.mat[2].clone();
                self.mat[1] += self.mat[3].clone();
                self.mat[4] += self.mat[6].clone();
                self.mat[5] += self.mat[7].clone();
            }
            Term::Rec => {
                self.mat.swap(0, 2);
                self.mat.swap(1, 3);
                self.mat.swap(4, 6);
                self.mat.swap(5, 7);
            }
            Term::Neg => {
                self.mat[0] = -self.mat[0].clone();
                self.mat[1] = -self.mat[1].clone();
                self.mat[4] = -self.mat[4].clone();
                self.mat[5] = -self.mat[5].clone();
            }
            Term::Inf => {
                self.mat[2] = self.mat[0].clone();
                self.mat[3] = self.mat[1].clone();
                self.mat[6] = self.mat[4].clone();
                self.mat[7] = self.mat[5].clone();
            }
            Term::Undefined => {
                self.mat = [
                    0.into(),
                    0.into(),
                    0.into(),
                    0.into(),
                    0.into(),
                    0.into(),
                    0.into(),
                    0.into(),
                ];
            }
        }
    }

    fn ingest_y(&mut self, y: Term) {
        match y {
            Term::Empty => {}
            Term::Ord => {
                if !self.mat[1].bit(0)
                    && !self.mat[3].bit(0)
                    && !self.mat[5].bit(0)
                    && !self.mat[7].bit(0)
                {
                    self.mat[1] >>= 1;
                    self.mat[3] >>= 1;
                    self.mat[5] >>= 1;
                    self.mat[7] >>= 1;
                } else {
                    self.mat[0] <<= 1;
                    self.mat[2] <<= 1;
                    self.mat[4] <<= 1;
                    self.mat[6] <<= 1;
                }
            }
            Term::DRec => {
                self.mat.swap(0, 1);
                self.mat.swap(2, 3);
                self.mat.swap(4, 5);
                self.mat.swap(6, 7);
                self.mat[0] += self.mat[1].clone();
                self.mat[2] += self.mat[3].clone();
                self.mat[4] += self.mat[5].clone();
                self.mat[6] += self.mat[7].clone();
            }
            Term::Rec => {
                self.mat.swap(0, 1);
                self.mat.swap(2, 3);
                self.mat.swap(4, 5);
                self.mat.swap(6, 7);
            }
            Term::Neg => {
                self.mat[0] = -self.mat[0].clone();
                self.mat[2] = -self.mat[2].clone();
                self.mat[4] = -self.mat[4].clone();
                self.mat[6] = -self.mat[6].clone();
            }
            Term::Inf => {
                self.mat[1] = self.mat[0].clone();
                self.mat[3] = self.mat[2].clone();
                self.mat[5] = self.mat[4].clone();
                self.mat[7] = self.mat[6].clone();
            }
            Term::Undefined => {
                self.mat = [
                    0.into(),
                    0.into(),
                    0.into(),
                    0.into(),
                    0.into(),
                    0.into(),
                    0.into(),
                    0.into(),
                ];
            }
        }
    }

    fn egest_z(&mut self) -> Term {
        // the 4 possibilities given x in [1, inf] and y in [1, inf]
        let mut n0 = self.mat[0].clone();
        let mut n1 = self.mat[0].clone() + self.mat[1].clone();
        let mut n2 = self.mat[0].clone() + self.mat[2].clone();
        let mut n3 =
            self.mat[0].clone() + self.mat[1].clone() + self.mat[2].clone() + self.mat[3].clone();
        let mut d0 = self.mat[4].clone();
        let mut d1 = self.mat[4].clone() + self.mat[5].clone();
        let mut d2 = self.mat[4].clone() + self.mat[6].clone();
        let mut d3 =
            self.mat[4].clone() + self.mat[5].clone() + self.mat[6].clone() + self.mat[7].clone();
        let signs = [
            crate::rationalize(&mut n0, &mut d0),
            crate::rationalize(&mut n1, &mut d1),
            crate::rationalize(&mut n2, &mut d2),
            crate::rationalize(&mut n3, &mut d3),
        ];
        // println!("{}/{}, {}/{}, {}/{}, {}/{}", n0, d0, n1, d1, n2, d2, n3, d3);

        let well_defined = !(signs.contains(&Sign::Plus) && signs.contains(&Sign::Minus))
        // i've spent a while debating whether to include this den_zeros check
        // brabec uses it
        // removing the check can cause non-speculative-driven retraction to happen
        // but it allows us to not choke on expressions like 5/(sqrt(2) * sqrt(2))
        // by allowing inputs with really long 1 streams to be processed without waiting for a 0 or oo
        // sometimes this produces longer outputs and sometimes shorter outputs
        // but always with less delay
        // anyways for now i'm gating it behind a const flag
            && (![d0.sign(), d1.sign(), d2.sign(), d3.sign()].contains(&Sign::NoSign)
                || MORE_AGGRESSIVE);

        // if the expression is undefined
        // then what else can we say
        if self.mat.iter().all(|i| *i == 0.into()) {
            return Term::Undefined;
        }

        // infinity!!!
        // (no 0/0 allowed)
        if d0 == 0.into()
            && d1 == 0.into()
            && d2 == 0.into()
            && d3 == 0.into()
            && n0 != 0.into()
            && n1 != 0.into()
            && n2 != 0.into()
            && n3 != 0.into()
        {
            // handling negative infinity :P
            if signs.iter().all(|s| *s == Sign::Minus) {
                self.mat[0] = -self.mat[0].clone();
                self.mat[1] = -self.mat[1].clone();
                self.mat[2] = -self.mat[2].clone();
                self.mat[3] = -self.mat[3].clone();
                return Term::Neg;
            }
            return Term::Inf;
        }

        // i wonder what happens if we ignore 0/0 if they're not all 0/0...
        // this'd allow us to speculate about e.g. 512-512 before it's done
        // then again, this speculation could go wildly wrong if it turned out to be 2^20-512 actually
        // and oo-oo IS undefined, so...
        // yeah ok let's not do that

        // most checks require us to be well defined
        // but we don't need well definedness for *everything*
        // we can maybe speculate a rec or ord
        if well_defined {
            // can't be a singularity if we're well defined
            // included for when we define ourselves out of a singularity through ingestion
            self.singularity = false;

            // the normal stuff
            if signs.iter().all(|s| *s == Sign::Minus) {
                self.egest(Term::Neg);
                return Term::Neg;
            } else if (n0.clone() >> 1) >= d0
                && (n1.clone() >> 1) >= d1
                && (n2.clone() >> 1) >= d2
                && (n3.clone() >> 1) >= d3
            {
                self.egest(Term::Ord);
                return Term::Ord;
            } else if n0 >= d0
                && n1 >= d1
                && n2 >= d2
                && n3 >= d3
                && (n0.clone() >> 1) < d0
                && (n1.clone() >> 1) < d1
                && (n2.clone() >> 1) < d2
                && (n3.clone() >> 1) < d3
            {
                self.egest(Term::DRec);
                return Term::DRec;
            } else if n0 < d0 && n1 < d1 && n2 < d2 && n3 < d3 {
                self.egest(Term::Rec);
                return Term::Rec;
            }
            // the following are SPECULATIVE
            else if d0 < n0
                && n0 < (d0.clone() << 2)
                && d1 < n1
                && n1 < (d1.clone() << 2)
                && d2 < n2
                && n2 < (d2.clone() << 2)
                && d3 < n3
                && n3 < (d3.clone() << 2)
            {
                self.egest(Term::Ord);
                return Term::Ord;
            } else if d0 < (n0.clone() << 1)
                && n0 < d0.clone() << 1
                && d1 < (n1.clone() << 1)
                && n1 < d1.clone() << 1
                && d2 < (n2.clone() << 1)
                && n2 < d2.clone() << 1
                && d3 < (n3.clone() << 1)
                && n3 < d3.clone() << 1
            {
                self.egest(Term::DRec);
                // we're launching ourselves into the singularity
                self.singularity = true;
                return Term::DRec;
            }
        } else {
            // not well defined :D
            if (!self.singularity || MORE_AGGRESSIVE)
                    // 0s ARE allowed in the denominator; we send to [-inf, -1) U (1, inf]
                    && n0.clone().into_parts().1 < d0.clone().into_parts().1
                    && n1.clone().into_parts().1 < d1.clone().into_parts().1
                    && n2.clone().into_parts().1 < d2.clone().into_parts().1
                    && n3.clone().into_parts().1 < d3.clone().into_parts().1
            {
                self.egest(Term::Rec);
                // we're launching ourselves into the singularity
                self.singularity = true;
                return Term::Rec;
            }
            if self.singularity || MORE_AGGRESSIVE {
                let a1: BigInt = d0.clone() << 1;
                let a2: BigInt = d1.clone() << 1;
                let a3: BigInt = d2.clone() << 1;
                let a4: BigInt = d3.clone() << 1;
                // brabec says to use [-infinity, -2) U (2, infinity]
                // i don't see any reason to not make that fully closed
                if a1.into_parts().1 <= n0.clone().into_parts().1
                    && a2.into_parts().1 <= n1.clone().into_parts().1
                    && a3.into_parts().1 <= n2.clone().into_parts().1
                    && a4.into_parts().1 <= n3.clone().into_parts().1
                {
                    self.egest(Term::Ord);
                    return Term::Ord;
                }
            }
        }

        Term::Empty
    }
}
