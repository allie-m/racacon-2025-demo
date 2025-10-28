use crate::{
    Term,
    unit::{Unit, UnitUnion, arith::Arith},
    workgroup::{UnitConcrete, UnitId, Workgroup},
};

// e^x = 1 + x + x^2/2! + x^3/3! + ...
// for the best results,
// input values 0 < x < 1
#[derive(Debug)]
pub struct ExpTaylor {
    inner: Workgroup,
    // we manually ingest x inputs into these
    init_term: UnitId,
    init_out: UnitId, // (y ingestion from init_term into here is automatic)
    // ingest
    taylor_terms: Vec<UnitId>,
    // each ingests from the previous out and the same level's taylor term
    outs: Vec<UnitId>,
    // we feed this x and the current last taylor (becomes last taylor term on add_layer)
    next_taylor: Arith,
    // we feed this x (becomes next_taylor on add_layer)
    next_next_taylor: Arith,
    // we feedback on next out with the last out term
    // that way they can seamlessly take over upon addition of another layer
    next_out: Arith,
    // for real xs, allows us to force add another layer
    real_egests_since_last_layer: u32,
}

impl Unit for ExpTaylor {
    fn ingest_x(&mut self, x: Term) {
        self.inner.get_unit_mut(self.init_term).inner.ingest_x(x);
        self.inner.get_unit_mut(self.init_term).inner.ingest_y(x);
        self.inner.get_unit_mut(self.init_out).inner.ingest_x(x);
        for id in self.taylor_terms.iter() {
            self.inner.get_unit_mut(*id).inner.ingest_x(x);
        }
        // cycle twice to both flush x and y through the taylor terms/outs
        self.inner.cycle();
        self.inner.cycle();
        // println!("ok so {:?}", self.inner.get_unit(*self.taylor_terms.last().unwrap()));

        // gotta keep the next taylor term apprised of developments in x
        self.next_taylor.ingest_x(x);
        self.next_next_taylor.ingest_x(x);
        // println!("next taylor term {:?}", self.next_taylor.mat);
    }

    fn ingest_y(&mut self, _y: Term) {
        // exp only takes x, so we ignore this
        // (we cycle inner twice on ingestions of x)
        // this shouldn't be invoked but i won't panic it
    }

    fn egest_z(&mut self) -> Term {
        self.inner.cycle();

        let last_taylor = self
            .inner
            .get_unit_mut(*self.taylor_terms.last().unwrap())
            .z;
        self.next_taylor.ingest_y(last_taylor);

        let out = self.inner.get_unit_mut(*self.outs.last().unwrap());
        let out_term = out.z;

        if out_term != Term::Empty {
            self.real_egests_since_last_layer += 1;
        }
        if out_term == Term::Inf
            // this is a made-up heuristic
            // this constant seems impossible to get right though
            // since the balance between previous taylor terms and new ones is hard to estimate
            || self.real_egests_since_last_layer >= 16 * self.taylor_terms.len() as u32
        {
            println!(
                "{} | {}",
                self.real_egests_since_last_layer,
                self.taylor_terms.len()
            );
            self.add_layer();
            self.real_egests_since_last_layer = 0;
            return Term::Empty;
        }

        // we are preemptively feedbacking our out term
        // into and from what will be the next out
        // so that it can seamlessly take over when we add another layer
        self.next_out.ingest_y(out_term);
        self.next_out.egest(out_term);

        out_term
    }
}

impl ExpTaylor {
    pub fn create() -> Self {
        let mut wg = Workgroup::create();
        let init_term = wg.add_unit(UnitConcrete {
            inner: UnitUnion::Arith(Arith::create([1, 0, 0, 0, 0, 0, 0, 2].map(|i| i.into()))),
            x: None,
            y: None,
            z: Default::default(),
        });
        let init_out = wg.add_unit(UnitConcrete {
            inner: UnitUnion::Arith(Arith::create([0, 1, 1, 1, 0, 0, 0, 1].map(|i| i.into()))),
            x: None,
            y: Some(init_term),
            z: Default::default(),
        });
        let first_term = wg.add_unit(UnitConcrete {
            inner: UnitUnion::Arith(Arith::create([1, 0, 0, 0, 0, 0, 0, 3].map(|i| i.into()))),
            x: None,
            y: Some(init_term),
            z: Default::default(),
        });
        let first_out = wg.add_unit(UnitConcrete {
            inner: UnitUnion::Arith(Arith::create([0, 1, 1, 0, 0, 0, 0, 1].map(|i| i.into()))),
            x: Some(first_term),
            y: Some(init_out),
            z: Default::default(),
        });
        Self {
            inner: wg,
            init_term,
            init_out,
            taylor_terms: vec![first_term],
            outs: vec![first_out],
            next_taylor: Arith::create([1, 0, 0, 0, 0, 0, 0, 4].map(|i| i.into())),
            next_next_taylor: Arith::create([1, 0, 0, 0, 0, 0, 0, 5].map(|i| i.into())),
            next_out: Arith::create([0, 1, 1, 0, 0, 0, 0, 1].map(|i| i.into())),
            real_egests_since_last_layer: 0,
        }
    }

    pub fn add_layer(&mut self) {
        let new_last_term = self.inner.add_unit(UnitConcrete {
            inner: UnitUnion::Arith(
                // Arith::create([1, 0, 0, 0, 0, 0, 0, self.taylor_terms.len() + 3].map(|i| i.into()))
                Arith::create(self.next_taylor.mat.clone()), // Arith::create([1, 0, 0, 0, 0, 0, 0, 1].map(|i| i.into()))
            ),
            x: None,
            y: Some(*self.taylor_terms.last().unwrap()), // (there will always be a last)
            z: Default::default(),
        });
        self.next_taylor.mat = self.next_next_taylor.mat.clone();
        self.next_next_taylor.mat[0] *= self.taylor_terms.len() + 4;
        self.next_next_taylor.mat[1] *= self.taylor_terms.len() + 4;
        self.next_next_taylor.mat[2] *= self.taylor_terms.len() + 4;
        self.next_next_taylor.mat[3] *= self.taylor_terms.len() + 4;
        self.next_next_taylor.mat[4] *= self.taylor_terms.len() + 5;
        self.next_next_taylor.mat[5] *= self.taylor_terms.len() + 5;
        self.next_next_taylor.mat[6] *= self.taylor_terms.len() + 5;
        self.next_next_taylor.mat[7] *= self.taylor_terms.len() + 5;
        self.taylor_terms.push(new_last_term);
        let out = self.inner.add_unit(UnitConcrete {
            inner: UnitUnion::Arith(Arith::create(self.next_out.mat.clone())),
            // (these are both guaranteed to exist)
            x: Some(*self.taylor_terms.last().unwrap()),
            y: Some(*self.outs.last().unwrap()),
            z: Default::default(),
        });
        self.outs.push(out);
    }
}
