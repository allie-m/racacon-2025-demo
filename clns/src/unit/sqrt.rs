use num_bigint::{BigInt, Sign};

use crate::{Term, unit::Unit};

pub const SQRT_SPECULATE: bool = true;

#[derive(Debug)]
pub struct Sqrt {
    mat: [BigInt; 8],
}

impl Sqrt {
    pub fn create() -> Self {
        Self {
            mat: [
                // TODO coefficients?
                // sadly we can't have a general purpose quadratic solver
                // but maybe we let the user multiply/add consts to x
                0.into(),
                1.into(),
                0.into(),
                0.into(),
                0.into(),
                0.into(),
                1.into(),
                0.into(),
            ],
        }
    }
}

// local feedback utils for implementation of egest_z
impl Sqrt {
    fn feedback_ord(&mut self) -> Term {
        // egest
        if self.mat[0].clone() % 2 == 0.into()
            && self.mat[1].clone() % 2 == 0.into()
            && self.mat[2].clone() % 2 == 0.into()
            && self.mat[3].clone() % 2 == 0.into()
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
        // ingest
        if self.mat[1].clone() % 2 == 0.into()
            && self.mat[3].clone() % 2 == 0.into()
            && self.mat[5].clone() % 2 == 0.into()
            && self.mat[7].clone() % 2 == 0.into()
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
        Term::Ord
    }
    fn feedback_drec(&mut self) -> Term {
        // egest
        self.mat[0] -= self.mat[4].clone();
        self.mat[1] -= self.mat[5].clone();
        self.mat[2] -= self.mat[6].clone();
        self.mat[3] -= self.mat[7].clone();
        self.mat.swap(0, 4);
        self.mat.swap(1, 5);
        self.mat.swap(2, 6);
        self.mat.swap(3, 7);
        // ingest
        self.mat.swap(0, 1);
        self.mat.swap(2, 3);
        self.mat.swap(4, 5);
        self.mat.swap(6, 7);
        self.mat[0] += self.mat[1].clone();
        self.mat[2] += self.mat[3].clone();
        self.mat[4] += self.mat[5].clone();
        self.mat[6] += self.mat[7].clone();
        Term::DRec
    }
    fn feedback_rec(&mut self) -> Term {
        // egest
        self.mat.swap(0, 4);
        self.mat.swap(1, 5);
        self.mat.swap(2, 6);
        self.mat.swap(3, 7);
        // ingest
        self.mat.swap(0, 1);
        self.mat.swap(2, 3);
        self.mat.swap(4, 5);
        self.mat.swap(6, 7);
        Term::Rec
    }
    fn feedback_neg(&mut self) -> Term {
        // egest
        // self.mat[0] = -self.mat[0].clone();
        self.mat[1] = -self.mat[1].clone();
        // self.mat[2] = -self.mat[2].clone();
        self.mat[3] = -self.mat[3].clone();
        // ingest
        // self.mat[0] = -self.mat[0].clone();
        // self.mat[2] = -self.mat[2].clone();
        self.mat[4] = -self.mat[4].clone();
        self.mat[6] = -self.mat[6].clone();
        Term::Neg
    }
}

impl Unit for Sqrt {
    fn ingest_x(&mut self, x: Term) {
        // println!("ingesting {:?}", x);
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

    // we don't ingest from y :P
    fn ingest_y(&mut self, _y: Term) {}

    fn egest_z(&mut self) -> Term {
        // if there's no y in the denominator,
        // whether x=1 or x=oo
        // then we've hit oo
        if self.mat[4] == 0.into() && self.mat[6] == 0.into() {
            // added to catch the case that the input is -oo
            // since that results in a state we don't catch as "undefined"
            if self.mat.iter().all(|i| *i <= 0.into()) {
                return Term::Undefined;
            }
            return Term::Inf;
        }

        // check that ad - bc <= 0 (for the ay+b cy+d mat)
        // the derivative is (ad - bc)/(cy + d)^2
        // since the roots (if they exist) are always on both sides of the asymptote,
        // the expression can't be defined if its derivative is always positive
        let defined_oo =
            self.mat[0].clone() * self.mat[5].clone() <= self.mat[1].clone() * self.mat[4].clone();
        let defined_1 = (self.mat[0].clone() + self.mat[2].clone())
            * (self.mat[5].clone() + self.mat[7].clone())
            <= (self.mat[1].clone() + self.mat[3].clone())
                * (self.mat[4].clone() + self.mat[6].clone());

        // normally we don't find ourselves in a situation where x straddles 0
        // so this is probably fine to be an or
        if !defined_oo || !defined_1 {
            // ok so there may be an issue with doing this
            // when inputs start negative and retract to be positive
            // for example, taking the sqrt of -111-1110
            // yields ø!!!!!øøøø-/1110∞
            // or something like 110-/111/1111 (+ to - to +)
            // the undefineds poison all downstream computations erroneously
            // and there is no way to retract
            // in practice i don't think this is a problem
            // because we tend to presume that inputs are positive, rather than negative
            // we prefer to speculatively egest / over -
            return Term::Undefined;
        }

        #[derive(Default)]
        struct LFTDecision {
            ord: bool,
            drec: bool,
            rec: bool,
            neg: bool,
            ord_spec: bool,
            drec_spec: bool,
            // i think we don't want rec_spec
            // because it applies when we're uncertain near 0
            // and when we're uncertain about 0 in sqrt, we simply bail
            // no undefinedness for us no thanky
            // as to ord_sing... idk we just don't seem to end up in that situation
            // even on erroneous egestion of 0
            // perhaps an artifact of only having one input
            // rec_spec: bool,
            // ord_sing: bool,
        }
        fn decide_lft(a: BigInt, b: BigInt, c: BigInt, d: BigInt) -> LFTDecision {
            let mut decision = LFTDecision::default();
            // asymptote :D
            let mut a_num = -d.clone();
            let mut a_den = c.clone();
            // if the asymptote doesn't exist, then we must be either a decreasing or constant line
            // since if c=0, we have ad <= 0
            let a_sign = (c != 0.into()).then_some(crate::rationalize(&mut a_num, &mut a_den));
            // now we've gotta sample y at some points :P
            let mut y2_num = 2 * a.clone() + b.clone();
            let mut y2_den = 2 * c.clone() + d.clone();
            let _ = crate::rationalize(&mut y2_num, &mut y2_den);
            let mut y1_num = a.clone() + b.clone();
            let mut y1_den = c.clone() + d.clone();
            let _ = crate::rationalize(&mut y1_num, &mut y1_den);
            let mut y0_num = b.clone();
            let mut y0_den = d.clone();
            let y0_sign = crate::rationalize(&mut y0_num, &mut y0_den);

            // println!("({}y + {})/({}y + {})", a, b, c, d);

            // y in [2, inf]
            if a_num >= 2 * a_den.clone() || y2_num >= 2 * y2_den.clone() {
                decision.ord = true;
                // decision.ord_sing = true;
            }
            // y in [1, 2)
            if a_num < 2 * a_den.clone()
                && y2_num < 2 * y2_den.clone()
                && (a_num >= a_den || y1_num >= y1_den)
            {
                decision.drec = true;
            }
            // y in [0, 1)
            if a_num < a_den && y1_num < y1_den && (a_num >= 0.into() || y0_num >= 0.into()) {
                decision.rec = true;
            }
            // y in [-inf, 0)
            if (a_sign.is_none() || a_sign == Some(Sign::Minus)) && y0_sign == Sign::Minus {
                decision.neg = true;
            }

            // speculative stuff
            let mut y4_num = 4 * a.clone() + b.clone();
            let mut y4_den = 4 * c.clone() + d.clone();
            let _ = crate::rationalize(&mut y4_num, &mut y4_den);
            let mut yh_num = a.clone() + 2 * b.clone();
            let mut yh_den = c.clone() + 2 * d.clone();
            let _ = crate::rationalize(&mut yh_num, &mut yh_den);
            // let mut ym2_num = 2 * -a.clone() + b.clone();
            // let mut ym2_den = 2 * -c.clone() + d.clone();
            // let _ = crate::rationalize(&mut ym2_num, &mut ym2_den);
            // let mut ym1_num = -a.clone() + b.clone();
            // let mut ym1_den = -c.clone() + d.clone();
            // let ym1_sign = crate::rationalize(&mut ym1_num, &mut ym1_den);
            // y in (1, 4)
            if a_num < 4 * a_den.clone()
                && y4_num < 4 * y4_den.clone()
                && (a_num >= a_den || y1_num >= y1_den)
            {
                decision.ord_spec = true;
            }
            // y in (1/2, 2)
            if a_num < 2 * a_den.clone()
                && y2_num < 2 * y2_den.clone()
                && (2 * a_num.clone() >= a_den.clone() || 2 * yh_num.clone() >= yh_den)
            {
                decision.drec_spec = true;
            }
            // // y in (-1, 1)
            // if a_num < a_den.clone()
            //     && y1_num < y1_den
            //     && ((a_num.clone().into_parts().1 < a_den.clone().into_parts().1)
            //     || (ym1_sign == Sign::Minus
            //         && ym1_num.into_parts().1 < ym1_den.clone().into_parts().1))
            // {
            //     decision.rec_spec = true;
            // }
            // // y in [-inf, 2]
            // if (a_sign.is_some() && -a_num >= 2 * a_den.clone()) || -ym2_num >= 2 * ym2_den.clone() {
            //     decision.ord_sing = true;
            // }

            decision
        }

        let decisions_oo = decide_lft(
            self.mat[0].clone(),
            self.mat[1].clone(),
            self.mat[4].clone(),
            self.mat[5].clone(),
        );
        let decisions_1 = decide_lft(
            self.mat[0].clone() + self.mat[2].clone(),
            self.mat[1].clone() + self.mat[3].clone(),
            self.mat[4].clone() + self.mat[6].clone(),
            self.mat[5].clone() + self.mat[7].clone(),
        );

        if decisions_oo.ord && decisions_1.ord {
            return self.feedback_ord();
        }
        if decisions_oo.drec && decisions_1.drec {
            return self.feedback_drec();
        }
        if decisions_oo.rec && decisions_1.rec {
            return self.feedback_rec();
        }
        if decisions_oo.neg && decisions_1.neg {
            return self.feedback_neg();
        }
        if SQRT_SPECULATE {
            if decisions_oo.ord_spec && decisions_1.ord_spec {
                return self.feedback_ord();
            }
            if decisions_oo.drec_spec && decisions_1.drec_spec {
                return self.feedback_drec();
            }
            // if decisions_oo.rec_spec && decisions_1.rec_spec {
            //     return self.feedback_rec();
            // }
            // if decisions_oo.ord_sing && decisions_1.ord_sing {
            //     println!("ord sing");
            //     return self.feedback_ord();
            // }
        }

        Term::Empty
    }
}
