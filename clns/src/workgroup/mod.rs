use crate::{
    Term,
    unit::{
        CLogs, Unit, UnitUnion, arith::Arith, cfrac::FromCFrac, cmp::Compare, int::Modulo,
        lft::Lft, sqrt::Sqrt,
    },
    workgroup::{exp::ExpTaylor, log2::Log2},
};
use std::collections::HashMap;

pub mod exp;
pub mod log2;
pub mod powu;

// TODO: ADD A WORKGROUP WRAPPER THAT HAS A UNIT IMPL
// wouldn't that be so fun??? it'd let us seamlessly integrate e.g. dynamically growable exp workgroups

// rather than the pull model of composing continued logarithm units together
// which is what i originally reached for
// i'm trying a push model --- we have a directed graph and do ingestion/egestion phases as a group

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WorkgroupPhase {
    IngestX,
    IngestY,
    EgestZ,
}

impl WorkgroupPhase {
    fn next(self) -> Self {
        use WorkgroupPhase::*;
        match self {
            IngestX => IngestY,
            IngestY => EgestZ,
            EgestZ => IngestX,
        }
    }
}

// each UnitId integer must be unique within a workgroup
// but not necessarily across workgroups
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UnitId(u32);

// we're not encoding the specific structure of the unit type
// (do you take x? do you take y? do you egest anything meaningful?)
// into our enums (cause that gets way too verbose)
// we expect anyone who accesses a unit from a unit id to understand
// the structure of that union
#[derive(Debug)]
pub struct UnitConcrete {
    pub inner: UnitUnion,
    pub x: Option<UnitId>,
    pub y: Option<UnitId>,
    pub z: Term,
}

#[derive(Debug)]
pub struct Workgroup {
    max_id: u32,
    units: HashMap<UnitId, UnitConcrete>,
    pub current_phase: WorkgroupPhase,
}

impl Workgroup {
    pub fn create() -> Workgroup {
        Workgroup {
            max_id: 0,
            units: HashMap::new(),
            current_phase: WorkgroupPhase::EgestZ,
        }
    }

    pub fn get_unit(&self, id: UnitId) -> &UnitConcrete {
        // TODO find a way to tether unitids to the workgroup which issues them
        self.units
            .get(&id)
            .expect("if we issued an id then the unit should exist :P")
    }

    pub fn get_unit_mut(&mut self, id: UnitId) -> &mut UnitConcrete {
        self.units
            .get_mut(&id)
            .expect("if we issued an id then the unit should exist :P")
    }

    fn new_id(&mut self) -> UnitId {
        let id = UnitId(self.max_id);
        self.max_id += 1;
        id
    }

    pub fn add_unit(&mut self, unit: UnitConcrete) -> UnitId {
        let id = self.new_id();
        self.units.insert(id, unit);
        id
    }

    pub fn add_arith(&mut self, unit: Arith, x: UnitId, y: UnitId) -> UnitId {
        let id = self.new_id();
        self.units.insert(
            id,
            UnitConcrete {
                inner: UnitUnion::Arith(unit),
                x: Some(x),
                y: Some(y),
                z: Default::default(),
            },
        );
        id
    }

    pub fn add_sqrt(&mut self, unit: Sqrt, x: UnitId) -> UnitId {
        let id = self.new_id();
        self.units.insert(
            id,
            UnitConcrete {
                inner: UnitUnion::Sqrt(unit),
                x: Some(x),
                y: None,
                z: Default::default(),
            },
        );
        id
    }

    pub fn add_from_cfrac(&mut self, unit: FromCFrac) -> UnitId {
        let id = self.new_id();
        self.units.insert(
            id,
            UnitConcrete {
                inner: UnitUnion::FromCFrac(unit),
                x: None,
                y: None,
                z: Default::default(),
            },
        );
        id
    }

    pub fn add_clogs(&mut self, unit: CLogs) -> UnitId {
        let id = self.new_id();
        self.units.insert(
            id,
            UnitConcrete {
                inner: UnitUnion::CLogs(unit),
                x: None,
                y: None,
                z: Default::default(),
            },
        );
        id
    }

    pub fn add_lft(&mut self, unit: Lft, x: Option<UnitId>) -> UnitId {
        let id = self.new_id();
        self.units.insert(
            id,
            UnitConcrete {
                inner: UnitUnion::Lft(unit),
                x,
                y: None,
                z: Default::default(),
            },
        );
        id
    }

    pub fn add_modulo(&mut self, unit: Modulo, x: UnitId, y: UnitId) -> UnitId {
        let id = self.new_id();
        self.units.insert(
            id,
            UnitConcrete {
                inner: UnitUnion::Modulo(unit),
                x: Some(x),
                y: Some(y),
                z: Default::default(),
            },
        );
        id
    }

    pub fn add_compare(&mut self, unit: Compare, x: UnitId, y: UnitId) -> UnitId {
        let id = self.new_id();
        self.units.insert(
            id,
            UnitConcrete {
                inner: UnitUnion::Compare(unit),
                x: Some(x),
                y: Some(y),
                z: Default::default(),
            },
        );
        id
    }

    // units that themselves wrap workgroups

    pub fn add_exp_taylor(&mut self, unit: ExpTaylor, x: UnitId) -> UnitId {
        let id = self.new_id();
        self.units.insert(
            id,
            UnitConcrete {
                inner: UnitUnion::ExpTaylor(unit),
                x: Some(x),
                y: None,
                z: Default::default(),
            },
        );
        id
    }

    pub fn add_log2(&mut self, unit: Log2, x: UnitId) -> UnitId {
        let id = self.new_id();
        self.units.insert(
            id,
            UnitConcrete {
                inner: UnitUnion::Log2(unit),
                x: Some(x),
                y: None,
                z: Default::default(),
            },
        );
        id
    }

    pub fn cycle(&mut self) {
        let keys = self.units.keys().cloned().collect::<Vec<_>>();
        for id in keys {
            match self.current_phase {
                WorkgroupPhase::IngestX => {
                    if let Some(x) = self.units[&id].x {
                        let term = self.units.get(&x).unwrap().z;
                        self.units.get_mut(&id).unwrap().inner.ingest_x(term);
                    }
                }
                WorkgroupPhase::IngestY => {
                    if let Some(y) = self.units[&id].y {
                        let term = self.units.get(&y).unwrap().z;
                        self.units.get_mut(&id).unwrap().inner.ingest_y(term);
                    }
                }
                WorkgroupPhase::EgestZ => {
                    let unit = self.units.get_mut(&id).unwrap();
                    unit.z = unit.inner.egest_z();
                }
            }
        }
        self.current_phase = self.current_phase.next();
    }
}

// // TODO is this a good abstraction to unify underneath?
// // embeddable workgroups are workgroups that mark which units are to be externally fed
// // and which unit can be treated as output
// // so that they can coherently implement Unit
// // and be embedded within another workgroup
// pub struct EmbeddableWorkgroup {
//     inner: Workgroup,
//     ingesters: Vec<UnitId>,
//     egester: UnitId,
// }

// impl Unit for EmbeddableWorkgroup {
//     fn ingest_x(&mut self, x: Term) {
//         for id in self.ingesters.iter() {
//             self.inner.get_unit_mut(*id).inner.ingest_x(x);
//         }
//         self.inner().cycle();
//     }

//     fn ingest_y(&mut self, y: Term) {
//         for id in self.ingesters.iter() {
//             self.inner.get_unit_mut(*id).inner.ingest_y(y);
//         }
//         self.inner().cycle();
//     }

//     fn egest_z(&mut self) -> Term {
//         self.inner().cycle();
//         self.inner.get_unit_mut(self.egester).inner.egest_z()
//     }
// }
