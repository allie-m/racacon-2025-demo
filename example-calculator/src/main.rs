use clns::{unit, workgroup};

mod parse;

fn main() {
    let stdin = std::io::stdin();
    let mut buf = String::new();
    // we're just not gonna deal with io errors
    // they'll get sent to stderr, whatever
    // enum OutFormat {
    //     Rational,
    //     // Decimal,
    //     ClogTerms,
    // }
    struct Cfg {
        egests: u32,
        // out_format: OutFormat,
    }
    let mut cfg = Cfg {
        egests: 100,
        // out_format: OutFormat::Rational,
    };
    while stdin.read_line(&mut buf).unwrap() > 0 {
        if buf.starts_with("cfg ") {
            let mut iter = buf.split(" ");
            let _ = iter.next().unwrap();
            match iter.next().map(|s| s.trim()) {
                Some(item) => match item {
                    "egests" => {
                        if let Some(Ok(item)) = iter.next().map(|i| i.trim().parse()) {
                            cfg.egests = item;
                            println!("Egests is now {}", cfg.egests);
                        }
                    }
                    // "rational" => cfg.out_format = OutFormat::Rational,
                    // "clog" => cfg.out_format = OutFormat::ClogTerms,
                    // TODO more
                    _ => {}
                },
                None => {}
            }
            buf.clear();
            continue;
        }
        let dag = match parse::roll_stack_expression(&buf) {
            Ok(dag) => dag,
            Err(e) => {
                eprintln!("Invalid stack expression: {:?}", e);
                buf.clear();
                continue;
            }
        };
        println!("{:?}", dag);
        let (mut wg, out, to_rat) = parse::stack_into_workgroup(dag);
        let mut terms = vec![];
        for _ in 0..cfg.egests * 3 {
            wg.cycle();
            if wg.current_phase == workgroup::WorkgroupPhase::IngestX {
                // println!("[{:?}]", wg.get_unit(out).z);
                terms.push(wg.get_unit(out).z);
                if let unit::UnitUnion::Compare(cmp) = &wg.get_unit(out).inner {
                    println!("{:?}", cmp.cmp());
                }
            }
        }
        let unit::UnitUnion::Lft(lft) = &wg.get_unit(to_rat).inner else {
            unreachable!()
        };
        for term in terms {
            print!("{:?}", term)
        }
        println!();
        println!(
            "{:?} | {:?} | {:?} | {:?}",
            lft.trunc(),
            lft.floor(),
            lft.ceil(),
            lft.round()
        );
        println!("{:?}", lft.intervals());
        buf.clear();
    }

    // let mut group = workgroup::Workgroup::create();
    // let val = group.add_from_rational(unit::rational::FromRational {
    //     num: 1u32.into(),
    //     den: 3u32.into(),
    //     sign: 1,
    // });
    // let exp_id = group.add_unit(workgroup::UnitConcrete {
    //     inner: unit::UnitUnion::Exp({
    //         let mut exp = workgroup::exp::exp();
    //         exp.add_layer();
    //         exp.add_layer();
    //         exp.add_layer();
    //         exp.add_layer();
    //         exp.add_layer();
    //         exp.add_layer();
    //         exp
    //     }),
    //     x: Some(val),
    //     y: None,
    //     z: Default::default(),
    // });
    // let out = group.add_into_rational(
    //     unit::rational::IntoRational {
    //         mat: [1.into(), 0.into(), 0.into(), 1.into()],
    //     },
    //     exp_id,
    // );
    // for _ in 0..1000 {
    //     if group.current_phase == workgroup::WorkgroupPhase::IngestX {
    //         println!("{:?}", group.get_unit(exp_id).z);
    //     }
    //     group.cycle();
    // }
    // println!(
    //     "{:?}",
    //     match &group.get_unit(out).inner {
    //         unit::UnitUnion::IntoRational(m) => m.to_rational(),
    //         _ => unreachable!(),
    //     }
    // );
    // // let a = unit::rational::FromRational {
    // //     num: 103u32.into(),
    // //     den: 1u32.into(),
    // //     sign: 1,
    // // };
    // let b = unit::rational::FromRational {
    //     num: 0u32.into(),
    //     den: 1u32.into(),
    //     sign: 1,
    // };
    // let sub = unit::arith::Arith::create([
    //     0.into(),
    //     1.into(),
    //     (-1).into(),
    //     0.into(),
    //     0.into(),
    //     0.into(),
    //     0.into(),
    //     1.into(),
    // ]);
    // let mut group = workgroup::Workgroup::create();
    // // let a_id = group.add_from_rational(a);
    // let a_id = group.add_from_cfrac(unit::cfrac::consts::pi());
    // let b_id = group.add_from_rational(b);
    // let arith_id = group.add_arith(sub, a_id, b_id);
    // let sqrt_id = group.add_sqrt(unit::sqrt::Sqrt::create(), arith_id);
    // let out = group.add_into_rational(
    //     unit::rational::IntoRational {
    //         mat: [1.into(), 0.into(), 0.into(), 1.into()],
    //     },
    //     sqrt_id,
    // );
    // for _ in 0..300 {
    //     group.cycle();
    //     // (we've just egested if we're ingestx)
    //     // if group.current_phase == workgroup::WorkgroupPhase::IngestX {
    //     //     println!("{:?}", group.get_unit(sqrt_id).z);
    //     //     // println!(
    //     //     //     "{:?}",
    //     //     //     match &group.get_unit(out).inner {
    //     //     //         unit::UnitUnion::IntoRational(m) => &m.mat,//m.to_rational(),
    //     //     //         _ => unreachable!(),
    //     //     //     }
    //     //     // );
    //     // }
    // }
    // println!(
    //     "{:?}",
    //     match &group.get_unit(out).inner {
    //         unit::UnitUnion::IntoRational(m) => m.to_rational(),
    //         _ => unreachable!(),
    //     }
    // );
}
