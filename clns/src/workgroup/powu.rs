// given an integer n
// and continued logarithm x
// calculates x^n
// by calculating x, x^2, x^4, x^8 (log2(p) for each power-of-2 x calculated)
// and composing them into
// should be O(x log n)

use crate::{
    unit::arith::Arith,
    workgroup::{UnitId, Workgroup},
};

// pub fn powu(wg: &mut Workgroup, x: UnitId, mut n: u32) {
//     let items = pow2(wg, x, n.ilog2());
//     for _ in 0..32 {
//         if n & 1 == 1 {
//             //
//         }
//         n >>= 1;
//     }
// }

// pub fn recombine(wg: &mut Workgroup, items: &[UnitId], mat: [num_bigint::BigInt; 8]) {
//     for a in items.chunks(2) {
//         wg.add_arith(Arith::create(mat.clone()), a[0], a[1]);
//     }
// }

// [x, x^2, x^4, ... x^(2^d)]
pub fn pow2(wg: &mut Workgroup, x: UnitId, d: u32) -> Vec<UnitId> {
    let mut items = vec![x];
    for _ in 0..d {
        let last = items.last().unwrap();
        items.push(wg.add_arith(
            Arith::create([1, 0, 0, 0, 0, 0, 0, 1].map(|i| i.into())),
            *last,
            *last,
        ));
    }
    items
}
