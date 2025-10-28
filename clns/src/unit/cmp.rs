use num_bigint::BigInt;

use crate::{
    Term,
    unit::{Unit, arith::Arith, lft::Lft},
};

#[derive(Debug)]
pub struct Compare {
    arith: Arith,
    lft: Lft,
}

impl Compare {
    pub fn create() -> Self {
        Compare {
            arith: Arith::create([
                0.into(),
                1.into(),
                (-1).into(),
                0.into(),
                0.into(),
                0.into(),
                0.into(),
                1.into(),
            ]),
            lft: Lft {
                mat: [1.into(), 0.into(), 0.into(), 1.into()],
                egest_enabled: false,
            },
        }
    }

    pub fn cmp(&self) -> (BigInt, BigInt) {
        // let (a, b) = self.lft.intervals();
        todo!()
    }
}

impl Unit for Compare {
    fn ingest_x(&mut self, x: Term) {
        self.arith.ingest_x(x);
    }

    fn ingest_y(&mut self, y: Term) {
        self.arith.ingest_y(y);
        self.lft.ingest_x(self.arith.egest_z());
    }

    // compare returns nothing :P
    fn egest_z(&mut self) -> Term {
        Term::Empty
    }
}
