use crate::{
    Term,
    unit::{Unit, UnitUnion, arith::Arith, lft::Lft},
    workgroup::{UnitConcrete, UnitId, Workgroup},
};

// this is the algorithm from https://mathr.co.uk/web/continued-logarithm.html#Logarithm
// the original version uses a continued logarithm n
// but i figure i can just do log2(x) * log2(n) and it's fine
// maybe i'll make a general-purpose version too
// this present implementation is very unoptimized
// and a little silly on account of me not designing this library super well
// this is definitely research quality code lmao
// so far this only works on input > 1 but it does work
#[derive(Debug)]
pub struct Log2 {
    wg: Workgroup,
    // hand-fed
    x: UnitId,
    // left > mediant > right
    left_lock: UnitId,
    right_lock: UnitId,
    mediant_lock: UnitId,
    // we hand-feed and hand-egest this lft
    // it's not part of the wg
    lft: Lft,
}

// hmm... i wonder if we can optimize this by checking the lft around power-of-2 boundaries
// if i can narrow the power-of-two interval then i can skip a lot of the steps
// maybe i can generalize this sorta bad impl into logn and then try my hand at optimizing the log2 version?

impl Log2 {
    // unlock everything
    // create new locked copies of them
    // with the mediant using the new mediant and one of the sides the old mediant
    fn add_layer(&mut self, left: bool) {
        // unlock everything
        let unit = self.wg.get_unit_mut(self.mediant_lock);
        if let UnitUnion::Lft(lft) = &mut unit.inner {
            lft.egest_enabled = true;
        }
            let unit = self.wg.get_unit_mut(self.left_lock);
            if let UnitUnion::Lft(lft) = &mut unit.inner {
                lft.egest_enabled = true;
            }
            let unit = self.wg.get_unit_mut(self.right_lock);
            if let UnitUnion::Lft(lft) = &mut unit.inner {
                lft.egest_enabled = true;
            }
        if left {
            // set the left to the old mediant
            self.left_lock = self.wg.add_lft(
                Lft {
                    mat: [1.into(), 0.into(), 0.into(), 1.into()],
                    egest_enabled: false,
                },
                Some(self.mediant_lock),
            );
            // create new mediant and right lock
            let mediant = self.wg.add_arith(
                Arith::create([1, 0, 0, 0, 0, 0, 0, 1].map(|i| i.into())),
                self.mediant_lock,
                self.right_lock,
            );
            self.mediant_lock = self.wg.add_lft(
                Lft {
                    mat: [1.into(), 0.into(), 0.into(), 1.into()],
                    egest_enabled: false,
                },
                Some(mediant),
            );
            self.right_lock = self.wg.add_lft(
                Lft {
                    mat: [1.into(), 0.into(), 0.into(), 1.into()],
                    egest_enabled: false,
                },
                Some(self.right_lock),
            );
        } else {
            // unlock the right
            let unit = self.wg.get_unit_mut(self.right_lock);
            if let UnitUnion::Lft(lft) = &mut unit.inner {
                lft.egest_enabled = true;
            }
            // set the right to the old mediant
            self.right_lock = self.wg.add_lft(
                Lft {
                    mat: [1.into(), 0.into(), 0.into(), 1.into()],
                    egest_enabled: false,
                },
                Some(self.mediant_lock),
            );
            // create new mediant and left lock
            let mediant = self.wg.add_arith(
                Arith::create([1, 0, 0, 0, 0, 0, 0, 1].map(|i| i.into())),
                self.mediant_lock,
                self.left_lock,
            );
            self.mediant_lock = self.wg.add_lft(
                Lft {
                    mat: [1.into(), 0.into(), 0.into(), 1.into()],
                    egest_enabled: false,
                },
                Some(mediant),
            );
            self.left_lock = self.wg.add_lft(
                Lft {
                    mat: [1.into(), 0.into(), 0.into(), 1.into()],
                    egest_enabled: false,
                },
                Some(self.left_lock),
            );
        }
    }
}

impl Unit for Log2 {
    // (x^a)/(2^c) <= 1 <= (x^b)/(2^d)
    fn ingest_x(&mut self, x: Term) {
        self.wg.get_unit_mut(self.x).inner.ingest_x(x);
        self.wg.cycle();
        self.wg.cycle();
    }

    fn ingest_y(&mut self, _: Term) {
        // log2 doesn't ingest from y :P
    }

    fn egest_z(&mut self) -> Term {
        self.wg.cycle();

        if self.lft.is_inf() { return self.lft.egest_z() }

        let med = self.wg.get_unit_mut(self.mediant_lock);
        if let UnitUnion::Lft(lft) = &med.inner {
            if lft.is_nonpositive() {
                return Term::Undefined;
            }
            // println!("{:?}", lft);
            let ((n1, d1), (n2, d2)) = lft.intervals();
            // x <= 1/2 TODO
            if (n1.clone() << 1) <= d1 && (n2.clone() << 1) <= d2 {
                // self.lft.mat[0] = 0.into();
                // self.lft.mat[1] = 0.into();
                // self.lft.mat[2] = 1.into();
                // self.lft.mat[3] = 1.into();
                // return self.lft.egest_z();
                println!("less than 1/2");
                return Term::Undefined;
            }
            // we're > 1
            if n1 > d1 && n2 > d2 {
                // we're > 1
                // left becomes mediant
                // println!("> 1");
                self.add_layer(true);
                self.lft.mat[0] += self.lft.mat[1].clone();
                self.lft.mat[2] += self.lft.mat[3].clone();
            }
            // we're < 1
            if n1 < d1 && n2 < d2 {
                // we're < 1
                // right becomes mediant
                // println!("< 1");
                self.add_layer(false);
                self.lft.mat[1] += self.lft.mat[0].clone();
                self.lft.mat[3] += self.lft.mat[2].clone();
            }
            // we're = 1
            if n1 == d1 && n2 == d2 {
                // we're exactly 1
                // doesn't matter which we choose cause we're on the way done
                // println!("= 1");
                self.add_layer(true);
                self.lft.mat[0] += self.lft.mat[1].clone();
                self.lft.mat[2] += self.lft.mat[3].clone();
                self.lft.mat[1] = self.lft.mat[0].clone();
                self.lft.mat[3] = self.lft.mat[2].clone();
            }
        } else {
            unreachable!()
        }

        self.lft.egest_z()
    }
}

impl Log2 {
    pub fn create() -> Self {
        let mut wg = Workgroup::create();
        let x = wg.add_unit(UnitConcrete {
            inner: UnitUnion::Lft(Lft {
                mat: [1.into(), 0.into(), 0.into(), 1.into()],
                egest_enabled: true,
            }),
            x: None,
            y: None,
            z: Default::default(),
        });
        let left = wg.add_lft(
            Lft {
                mat: [1.into(), 0.into(), 0.into(), 1.into()],
                egest_enabled: true,
            },
            Some(x),
        );
        let left_lock = wg.add_lft(
            Lft {
                mat: [1.into(), 0.into(), 0.into(), 1.into()],
                egest_enabled: false,
            },
            Some(left),
        );
        let right = wg.add_lft(
            Lft {
                mat: [1.into(), 1.into(), 2.into(), 2.into()],
                egest_enabled: true,
            },
            None,
        );
        let right_lock = wg.add_lft(
            Lft {
                mat: [1.into(), 0.into(), 0.into(), 1.into()],
                egest_enabled: false,
            },
            Some(right),
        );
        let mediant = wg.add_arith(
            Arith::create([1, 0, 0, 0, 0, 0, 0, 1].map(|i| i.into())),
            left,
            right,
        );
        let mediant_lock = wg.add_lft(
            Lft {
                mat: [1.into(), 0.into(), 0.into(), 1.into()],
                egest_enabled: false,
            },
            Some(mediant),
        );
        Self {
            wg,
            x,
            left_lock,
            right_lock,
            mediant_lock,
            lft: Lft {
                mat: [0.into(), 1.into(), 1.into(), 0.into()],
                egest_enabled: true,
            },
        }
    }
}
