use std::str::FromStr;

use clns::{unit, workgroup};

use num_bigint::{BigInt, BigUint};

#[derive(Debug)]
pub enum Node {
    OneChild(OneChild, Box<Node>),
    TwoChildren(TwoChildren, Box<Node>, Box<Node>),
    // zero children
    Decimal { word: BigUint, pow: u16 }, // val = word * 10^(-pow)
    Constant { kind: Constant },
    CLog { items: Vec<clns::Term> },
    CFrac { terms: Vec<i64> },
}

#[derive(Clone, Copy, Debug)]
pub enum OneChild {
    Abs,
    Sqrt,
    Exp,
    Ln,
    Log2,
}

#[derive(Clone, Copy, Debug)]
pub enum TwoChildren {
    Add,
    Sub,
    Mul,
    Div,
    Floor,
    Ceil,
    Round,
    Mod,
    Pow, // (base, exp)
    Log, // (base, exp)
    Compare,
}

#[derive(Clone, Copy, Debug)]
pub enum Constant {
    Pi,
    // Tau,
    E,
    // Phi,
    Inf,
}

#[derive(Clone, Debug)]
pub enum RollExprError {
    #[allow(unused)]
    InvalidToken(String),
    InvalidDecimal,
    InvalidName,
    EmptyStack,
}

pub fn stack_into_workgroup(
    node: Box<Node>,
) -> (workgroup::Workgroup, workgroup::UnitId, workgroup::UnitId) {
    let mut wg = workgroup::Workgroup::create();
    fn inner(node: Box<Node>, wg: &mut workgroup::Workgroup) -> workgroup::UnitId {
        match *node {
            Node::Constant { kind: Constant::E } => wg.add_from_cfrac(unit::cfrac::consts::e()),
            Node::Constant { kind: Constant::Pi } => wg.add_from_cfrac(unit::cfrac::consts::pi()),
            Node::Constant {
                kind: Constant::Inf,
            } => wg.add_clogs(unit::CLogs {
                terms: Box::new(|| clns::Term::Inf),
            }),
            Node::CLog { items } => wg.add_clogs(unit::CLogs {
                terms: Box::new({
                    let mut iter = items.into_iter();
                    move || iter.next().unwrap_or(clns::Term::Empty)
                }),
            }),
            Node::CFrac { terms } => wg.add_from_cfrac(unit::cfrac::FromCFrac {
                mat: [1, 0, 0, 1].map(|i| i.into()),
                iter: Box::new(terms.into_iter().map(|t| (t, 1, 1, 1)))
            }),
            Node::Decimal { word, pow } => wg.add_lft(
                unit::lft::Lft {
                    mat: [
                        word.clone().into(),
                        word.into(),
                        BigInt::from(10u32).pow(pow as u32),
                        BigInt::from(10u32).pow(pow as u32),
                    ],
                    egest_enabled: true,
                },
                None,
            ),
            Node::OneChild(kind, node) => {
                let child = inner(node, wg);
                match kind {
                    OneChild::Sqrt => wg.add_sqrt(unit::sqrt::Sqrt::create(), child),
                    OneChild::Exp => wg.add_exp_taylor(
                        {
                            let exp = workgroup::exp::ExpTaylor::create();
                            // exp.add_layer();
                            exp
                        },
                        child,
                    ),
                    OneChild::Log2 => wg.add_log2(workgroup::log2::Log2::create(), child),
                    _ => panic!(),
                }
            }
            Node::TwoChildren(kind, n1, n2) => {
                let x = inner(n1, wg);
                let y = inner(n2, wg);
                match kind {
                    TwoChildren::Add => wg.add_arith(
                        unit::arith::Arith::create([0, 1, 1, 0, 0, 0, 0, 1].map(|i| i.into())),
                        x,
                        y,
                    ),
                    TwoChildren::Sub => wg.add_arith(
                        unit::arith::Arith::create([0, 1, -1, 0, 0, 0, 0, 1].map(|i| i.into())),
                        x,
                        y,
                    ),
                    TwoChildren::Mul => wg.add_arith(
                        unit::arith::Arith::create([1, 0, 0, 0, 0, 0, 0, 1].map(|i| i.into())),
                        x,
                        y,
                    ),
                    TwoChildren::Div => wg.add_arith(
                        unit::arith::Arith::create([0, 1, 0, 0, 0, 0, 1, 0].map(|i| i.into())),
                        x,
                        y,
                    ),
                    TwoChildren::Mod => wg.add_modulo(unit::int::Modulo::create(), x, y),
                    TwoChildren::Compare => wg.add_compare(unit::cmp::Compare::create(), x, y),
                    _ => todo!(),
                }
            }
        }
    }
    let output = inner(node, &mut wg);
    let to_rat = wg.add_lft(
        unit::lft::Lft {
            mat: [1.into(), 0.into(), 0.into(), 1.into()],
            egest_enabled: false,
        },
        Some(output),
    );
    (wg, output, to_rat)
}

// rolls a stack expression into a DAG
// items leftover at the bottom of the stack are ignored
pub fn roll_stack_expression(expr: &str) -> Result<Box<Node>, RollExprError> {
    let mut stack = vec![];
    for token in expr.trim().split(" ") {
        let token = token.trim();
        if token.is_empty() {
            continue;
        }
        match token {
            // binary ops
            "+" | "-" | "*" | "/" | "%" | "^" | "log" | "floor" | "ceil" | "round" | "cmp" => {
                let t1 = stack.pop().ok_or(RollExprError::EmptyStack)?;
                let t2 = stack.pop().ok_or(RollExprError::EmptyStack)?;
                stack.push(Box::new(Node::TwoChildren(
                    match token {
                        "+" => TwoChildren::Add,
                        "-" => TwoChildren::Sub,
                        "*" => TwoChildren::Mul,
                        "/" => TwoChildren::Div,
                        "%" => TwoChildren::Mod,
                        "^" => TwoChildren::Pow,
                        "log" => TwoChildren::Log,
                        "floor" => TwoChildren::Floor,
                        "ceil" => TwoChildren::Ceil,
                        "round" => TwoChildren::Round,
                        "cmp" => TwoChildren::Compare,
                        _ => unreachable!(),
                    },
                    t1,
                    t2,
                )));
            }
            // unary ops
            "abs" | "sqrt" | "exp" | "ln" | "log2" => {
                let top = stack.pop().ok_or(RollExprError::EmptyStack)?;
                stack.push(Box::new(Node::OneChild(
                    match token {
                        "abs" => OneChild::Abs,
                        "sqrt" => OneChild::Sqrt,
                        "exp" => OneChild::Exp,
                        "ln" => OneChild::Ln,
                        "log2" => OneChild::Log2,
                        _ => unreachable!(),
                    },
                    top,
                )));
            }
            // constants
            "pi" | "e" | "inf" => stack.push(Box::new(Node::Constant {
                kind: match token {
                    "pi" => Constant::Pi,
                    "e" => Constant::E,
                    "inf" => Constant::Inf,
                    _ => unreachable!(),
                },
            })),
            other => {
                if other.chars().all(|c| !c.is_alphanumeric() && c != '.') {
                    return Err(RollExprError::InvalidToken(token.to_owned()));
                }
                // TODO RATIONAL INPUTS!!!!
                if other.starts_with("f:") {
                    let mut terms = vec![];
                    for term in other[2..].split(",") {
                        // TODO optional generalized cfrac inputs
                        if let Ok(t) = term.parse() {
                            terms.push(t);
                        }
                    }
                    stack.push(Box::new(Node::CFrac { terms }));
                } else if other.starts_with("c") {
                    // clog terms
                    let mut items = vec![];
                    for c in other.chars() {
                        match c {
                            '1' => items.push(clns::Term::Ord),
                            '0' => items.push(clns::Term::DRec),
                            '/' => items.push(clns::Term::Rec),
                            '-' => items.push(clns::Term::Neg),
                            'ø' => items.push(clns::Term::Empty),
                            '∞' => items.push(clns::Term::Inf),
                            '!' => items.push(clns::Term::Undefined),
                            _ => {}
                        }
                    }
                    stack.push(Box::new(Node::CLog { items }))
                } else if other.chars().all(|c| c.is_numeric() || c == '.') {
                    // decimal
                    let points = other.chars().filter(|c| *c == '.').count();
                    if points > 1 || other.chars().count() == points {
                        return Err(RollExprError::InvalidDecimal);
                    }
                    let zero = !other.contains(|c| match c {
                        '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => true,
                        _ => false,
                    });
                    if !zero {
                        let pow = other.len()
                            - other.find('.').unwrap_or(other.len())
                            - other.contains(".") as usize;
                        let pow = pow.try_into().expect("if your pow can't fit into a u16 then you need to reevaluate your life");
                        let word = other.chars().filter(|c| *c != '.').collect::<String>();
                        stack.push(Box::new(Node::Decimal {
                            word: BigUint::from_str(&word).unwrap(),
                            pow,
                        }));
                    } else {
                        stack.push(Box::new(Node::Decimal {
                            word: BigUint::ZERO,
                            pow: 0,
                        }))
                    }
                } else {
                    return Err(RollExprError::InvalidName);
                }
            }
        }
    }
    stack.pop().ok_or(RollExprError::EmptyStack)
}
