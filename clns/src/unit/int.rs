use crate::{
    Term,
    unit::{Unit, arith::Arith, lft::Lft},
};

// as defined here, modulus computes
// r = x - trunc(x / y) * y
// (we take q = trunc(x / y))
// we use trunc instead of floor or ceil or round because you only have to decide on one boundary
// really, all these integer ops sorta suck for continued logarithms
// because they require deciding on a firm boundary and sticking with it
// if speculation's turned off then you might never decide
// and if it's turned on then you might decide wrong
// for continuous things like arithmetic, we can simply retract and get arbitrarily close
// for discontinuous things like this, we can simply be wrong, and there's no way around it
// without demanding that we wait until inputs are fully ingested and settled
#[derive(Debug)]
pub struct Modulo {
    div: Arith,
    quotient: Lft,
    x: Lft,
    y: Lft,
    out: Option<Arith>,
}

impl Modulo {
    pub fn create() -> Self {
        Self {
            div: Arith::create([
                0.into(),
                1.into(),
                0.into(),
                0.into(),
                0.into(),
                0.into(),
                1.into(),
                0.into(),
            ]),
            quotient: Lft {
                egest_enabled: false,
                ..Lft::identity()
            },
            x: Lft::identity(),
            y: Lft::identity(),
            out: None,
        }
    }
}

impl Unit for Modulo {
    fn ingest_x(&mut self, x: Term) {
        self.div.ingest_x(x);
        self.x.ingest_x(x);
        if let Some(out) = &mut self.out {
            let nxt = self.x.egest_z();
            out.ingest_x(nxt);
        }
    }

    fn ingest_y(&mut self, y: Term) {
        self.div.ingest_y(y);
        self.quotient.ingest_x(self.div.egest_z());
        self.y.ingest_x(y);
        if let Some(out) = &mut self.out {
            let nxt = self.y.egest_z();
            out.ingest_y(nxt);
        }
        // wait... what if the value changes due to retraction
        // ok so it seems like trunc can sometimes be wrong
        // if we misspeculate being on the wrong side of an integer boundary
        // although we shouldn't ever be off by more than 1 i think
        // cause that only happens if the input is VERY malformed
        // println!("{:?}", self.quotient.trunc());
        if let None = self.out {
            // if x or y is undefined, then so are we
            // also, oo % x = !!!!
            if self.quotient.is_inf() || self.quotient.is_undefined() {
                self.out = Some(Arith::create([
                    0.into(),
                    0.into(),
                    0.into(),
                    0.into(),
                    0.into(),
                    0.into(),
                    0.into(),
                    0.into(),
                ]))
            }
            // i'm special casing it so that x % +-oo = x
            // unless x = +-oo, since +-oo % +-oo = !
            if !self.x.is_inf() && self.y.is_inf() {
                // xy/y; y=oo so it just becomes x
                // the other expr gives (x - 0y)/1
                // which evals to !!! at y=oo because of the implied leading xy term
                self.out = Some(Arith::create([
                    1.into(),
                    0.into(),
                    0.into(),
                    0.into(),
                    0.into(),
                    0.into(),
                    1.into(),
                    0.into(),
                ]));
            }
            if let Some(val) = self.quotient.trunc() {
                println!("we're settling on trunc val {}", val);
                self.out = Some(Arith::create([
                    0.into(),
                    1.into(),
                    -val,
                    0.into(),
                    0.into(),
                    0.into(),
                    0.into(),
                    1.into(),
                ]));
            }
        }
    }

    fn egest_z(&mut self) -> Term {
        self.out
            .as_mut()
            .map(|arith| arith.egest_z())
            .unwrap_or(Term::Empty)
    }
}
