#![feature(box_into_inner)]
use cached::proc_macro::cached;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Insn {
    Inp(Var),
    Expr(Op, Var, Arg)
}
fn counter(mut x: usize) -> usize {
    let mut out = 0;
    while x != 0 {
        x /= 10;
        x += 1;
    }
    out
}

use Insn::*;
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Op {
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}
use Op::*;
impl Op {
    fn run(&self, l: i64, r: i64) -> i64 {
        match *self {
            Add => l + r,
            Mul => l * r,
            // TODO this could panic
            Div => l / r,
            Mod => l % r,
            Eql => if l == r { 1 } else {0 }
        }
    }
}
impl Op {
    fn from_str(x: &str) -> Op {
        match x {
            "add" => Add,
            "mul" => Mul,
            "div" => Div,
            "mod" => Mod,
            "eql" => Eql,
            _ => panic!("bad op {}", x)
        }
    }
}
type Var = char;
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Arg {
    Lit(i64),
    Var(Var)
}
impl Arg {
    fn parse(arg: &str) -> Arg {
        use Arg::*;
        if let Ok(l) = i64::from_str(arg) {
            Lit(l)
        } else {
            assert!(arg.len() == 1);
            Var(arg.chars().next().unwrap())
        }
    }
}

type Input = Vec<Insn>;
fn parse(input: impl Iterator<Item = String>) -> Input {
    input.map(|line| {
        let mut tok = aoc::tokenize(&line);
        let ops = tok.next().unwrap();
        if ops == "inp" {
            Inp(tok.next().unwrap().chars().next().unwrap())
        } else {
            Expr(Op::from_str(ops), tok.next().unwrap().chars().next().unwrap(), Arg::parse(tok.next().unwrap()))
        }
    }).collect()
}
use itertools::Itertools;
use std::collections::{HashMap, HashSet, BinaryHeap};
use sscanf::scanf;
use std::str::FromStr;
const INPUT: &str = include_str!("../../inputs/2021/24");
fn main() {
    println!(
        "A: {}\tB: {}",
        solve_a(parse(aoc::str_input(INPUT))),
        solve_b(parse(aoc::str_input(INPUT)))
    );
}
#[derive(Default, Debug, Clone, Copy, Ord, PartialEq, PartialOrd, Eq, Hash)]
struct Conc {
    vars: [i64; 4],
}
fn c2i(x: char) -> usize {
    x as usize - 'w' as usize
}
impl Conc {
    fn new() -> Self {
        Default::default()
    }
    fn run(&mut self, insn: &Insn) -> Option<Vec<(usize, Self)>> {
        match *insn {
               Inp(ref var) => {
                   let mut out = Vec::new();
                   for test_val in (1..10).rev() {
                       let mut test_conc = self.clone();
                       test_conc.vars[c2i(*var)] = test_val as i64;
                       out.push((test_val, test_conc));
                   }
                   Some(out)
               }
               Expr(op, ref out, ref arg) => {
                   let l = self.vars[c2i(*out)];
                   let r = match *arg {
                       Arg::Var(x) => self.vars[c2i(x)],
                       Arg::Lit(l) => l,
                   };
                   self.vars[c2i(*out)] = op.run(l, r);
                   None
               }
            }
    }
}
 
#[derive(Default, Debug, Clone)]
struct State {
    vars: HashMap<char, Val>,
    next_sym: u8,
}
impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for var in ['w', 'x', 'y', 'z'] {
            writeln!(f, "{} = {}", var, self.vars[&var])?;
        }
        Ok(())
    }
}
impl State {
    fn new() -> Self {
        let mut q: Self = Default::default();
        q.vars.insert('w', Val::Lit(0));
        q.vars.insert('x', Val::Lit(0));
        q.vars.insert('y', Val::Lit(0));
        q.vars.insert('z', Val::Lit(0));
        q
    }
    fn exec(&mut self, insn: &Insn) {
        match *insn {
            Inp(ref var) => {
                self.vars.insert(*var, Val::Sym(self.next_sym));
                self.next_sym += 1;
            }
            Expr(op, out, ref arg) => {
               let qa = match arg {
                   Arg::Lit(v) => Val::Lit(*v),
                   Arg::Var(c) => self.vars[&c].clone(),
               };
               self.vars.insert(out, simplify(Val::Expr(op, Box::new(self.vars[&out].clone()), Box::new(qa))));
            }
        }
    }
}

#[cached]
fn bound(v: Val) -> (i64, i64) {
    use Val::*;
    let out = match v {
        Sym(_) => (1, 9),
        Lit(x) => (x, x),
        Expr(op, ref l, ref r) => {
            let (ll, lh) = bound(*l.clone());
            let (rl, rh) = bound(*r.clone());
            match op {
                Add => (ll + rl, lh + rh),
                Mul => {
                    let nums = [ll * rl, ll * rh, lh * rl, lh * rh];
                    (*nums.iter().min().unwrap(), *nums.iter().max().unwrap())
                }
                Mod => {
                    assert!(ll >= 0);
                    assert!(rl >= 0);
                    assert!(rl == rh);
                    if ll == lh {
                        (ll % rl, ll % rl)
                    } else if lh < rl {
                        (ll, lh)
                    } else {
                        (0, rl - 1)
                    }
                }
                Div => {
                    // Div only happens with literals in the program
                    assert_eq!(rl, rh);
                    (ll / rl, lh / rl)
                }
                Eql => {
                    if (ll == lh) && (rl == rh) {
                        // Concrete
                        if ll == rl {
                            (1, 1)
                        } else {
                            (0, 0)
                        }
                    } else if rl > lh || ll > rh {
                        // Ranges don't overlap
                        (0, 0)
                    }  else {
                        (0, 1)
                    }
                }
            }
        }
    };
    //println!("Bounded\n{}\nby\n{:?}", v, out);
    out
}

#[cached]
fn simplify(v: Val) -> Val {
    let v2 = reduce(v);
    let (l, h) = bound(v2.clone());
    if l == h {
        Val::Lit(l)
    } else {
        v2
    }
}

#[cached]
fn reduce(v: Val) -> Val {
    use Val::*;
    match v {
        Sym(x) => Sym(x),
        Lit(x) => Lit(x),
        Expr(op, ref l, ref r) => {
            let lsa = simplify(*l.clone());
            let rsa = simplify(*r.clone());
            if let Lit(ll) = lsa {
                if let Lit(rr) = rsa {
                    return Lit(op.run(ll, rr))
                }
            }
            let ls = Box::new(lsa);
            let rs = Box::new(rsa);
            match op {
                Mul => {
                    if ls.is_zero() || rs.is_zero() {
                        Lit(0)
                    } else if ls.is_one() {
                        *rs
                    } else if rs.is_one() {
                        *ls
                    } else if rs < ls {
                        Expr(op, rs, ls)
                    } else {
                        Expr(op, ls, rs)
                    }
                }
                Add => {
                    if let Lit(lv) = *ls {
                        if let Expr(Add, ref il, ref ir) = *rs {
                            if let Lit(ilv) = **il {
                                return simplify(Expr(Add, Box::new(Lit(ilv + lv)), ir.clone()))
                            }
                            if let Lit(ilr) = **ir {
                                return simplify(Expr(Add, il.clone(), Box::new(Lit(lv + ilr))))
                            }
                        }
                    }
                    if let Lit(rv) = *rs {
                        if let Expr(Add, ref il, ref ir) = *ls {
                            if let Lit(ilv) = **il {
                                return simplify(Expr(Add, Box::new(Lit(ilv + rv)), ir.clone()))
                            }
                            if let Lit(ilr) = **ir {
                                return simplify(Expr(Add, il.clone(), Box::new(Lit(rv + ilr))))
                            }
                        }
                    }

                    if ls.is_zero() {
                        *rs
                    } else if rs.is_zero() {
                        *ls
                    } else if rs < ls {
                        Expr(op, rs, ls)
                    } else {
                        Expr(op, ls, rs)
                    }
                }
                Div => {
                    if let Lit(rv) = *rs {
                        if let Expr(Add, ref il, ref ir) = *ls {
                            let dil = Expr(Div, il.clone(), Box::new(Lit(rv)));
                            let dir = Expr(Div, ir.clone(), Box::new(Lit(rv)));
                            let sil = simplify(dil.clone());
                            let sir = simplify(dir.clone());
                            if sil != dil || dir != sir {
                                return simplify(Expr(Add, Box::new(sil), Box::new(sir)))
                            }
                        }
                        if let Expr(Mul, ref il, ref ir) = *ls {
                            if let Lit(v) = **il {
                                if rv == v {
                                    return *ir.clone()
                                }
                            }
                            if let Lit(v) = **ir {
                                if rv == v {
                                    return *il.clone()
                                }
                            }
                        }
                    }

                    if rs.is_one() {
                        *ls
                    } else if ls == rs {
                        Lit(1)
                    } else {
                        Expr(op, ls, rs)
                    }
                }
                Mod => {
                    if let Lit(rv) = *rs {
                        let (_, lh) = bound(*ls.clone());
                        if lh < rv {
                            return *ls.clone();
                        }
                        if let Expr(Add, ref il, ref ir) = *ls {
                            let dil = Expr(Mod, il.clone(), Box::new(Lit(rv)));
                            let dir = Expr(Mod, ir.clone(), Box::new(Lit(rv)));
                            let sil = simplify(dil.clone());
                            let sir = simplify(dir.clone());
                            if sil != dil || dir != sir {
                                let cand = Expr(Mod, Box::new(simplify(Expr(Add, Box::new(sil), Box::new(sir)))), Box::new(Lit(rv)));
                                if cand != v {
                                    return simplify(cand)
                                }
                            }
                        }
                        if let Expr(Mul, ref il, ref ir) = *ls {
                            if let Lit(v) = **il {
                                if rv == v {
                                    return Lit(0)
                                }
                            }
                            if let Lit(v) = **ir {
                                if rv == v {
                                    return Lit(0)
                                }
                            }
                        }
                    }

                    if rs.is_one() {
                        Lit(0)
                    } else if ls.is_zero() {
                        Lit(0)
                    } else {
                        Expr(op, ls, rs)
                    }
                }
                Eql => {
                    if ls == rs {
                        return Lit(1)
                    }
                    match (&*ls, &*rs) {
                        (&Sym(_), &Lit(ref i)) | (&Lit(ref i), &Sym(_)) => {
                        if *i < 1 || *i > 9 {
                            return Lit(0)
                        } else {
                            return Lit(1)
                        }
                    }
                        _ => ()
                    }

                    if rs < ls {
                        Expr(op, rs, ls)
                    } else {
                        Expr(op, ls, rs)
                    }
                }
            }
        }
    }
}
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Val {
    Sym(u8),
    Lit(i64),
    Expr(Op, Box<Val>, Box<Val>),
}
impl Val {
    fn is_one(&self) -> bool {
        *self == Val::Lit(1)
    }
    fn is_zero(&self) -> bool {
        *self == Val::Lit(0)
    }
    fn subst(&mut self, id: u8, val: i64) {
        use Val::*;
        match self {
            Sym(idx) => if *idx == id {
                *self = Lit(val);
            }
            Lit(_) => (),
            Expr(_, l, r) => {
                l.subst(id, val);
                r.subst(id, val);
            }
        }
    }

}
use std::fmt;
impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Val::*;
        match *self {
            Sym(i) => write!(f, "v{}", i),
            Lit(i) => write!(f, "{}", i),
            Expr(op, ref l, ref r) => write!(f, "({} {} {})", l, op, r)
        }
    }
}
impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Op::*;
        let x = match *self {
            Mul => "*",
            Add => "+",
            Div => "/",
            Mod => "%",
            Eql => "==",
        };
        write!(f, "{}", x)
    }
}

fn solve_a(mut input: Input) -> usize {
    return 0;
    use std::cmp::Reverse;
    let state = Conc::new();
    let mut wq = BinaryHeap::new();
    let mut visited = HashSet::new();
    wq.push((0, Reverse(0), Reverse(0), state));
    while let Some((mut idx, Reverse(serial), Reverse(z), mut state)) = wq.pop() {
        if idx >= input.len() && state.vars[c2i('z')] == 0 {
                return serial;
        }
        if idx >= input.len() {
            continue;
        }
        if visited.insert((idx, state)) {
          let new_states = loop {
              let out = state.run(&input[idx]);
              idx += 1;
              if let Some(new_states) = out {
                  break new_states;
              }
              if idx >= input.len() {
                  break vec![(0, state)];
              }
          };
          for (digit, mut s2) in new_states {
              s2.vars[c2i('x')] = 0;
              s2.vars[c2i('y')] = 0;
              let ser2 = serial * 10 + digit;
              wq.push((idx, Reverse(ser2), Reverse(s2.vars[c2i('z')]), s2));
          }
        }
    }
    panic!("no solution found")
}
fn search2(expr: Val, mut digits: Vec<i64>) -> Option<Vec<i64>> {
    //println!("Digits: {:?}", digits);
    let expr = simplify(expr);
    let (l, h) = bound(expr.clone());
    if l > 0 || h < 0 {
        //println!("Infeasible range: {:?}/{:?}", l, h);
        return None
    }
    if digits.len() == 14 {
        if (l, h) == (0, 0) {
            return Some(digits)
        } else {
            return None
        }
    }
    let n = digits.len();
    digits.push(0);
    for digit in 1..10 {
        if n == 0 {
            println!("Checking first digit {}", digit);
        }
        digits[n] = digit;
        let mut q = expr.clone();
        q.subst(n as u8, digit);
        if let Some(z) = search2(q, digits.clone()) {
            return Some(z)
        }
    }
    None
}
fn solve_b(mut input: Input) -> usize {
    let mut state = State::new();
    for (step, insn) in input.into_iter().enumerate() {
        println!("step {:?}: {:?}", step, insn);
        state.exec(&insn);
        println!("{}", state);
    }
    println!("SEARCHING");
    let expr = state.vars[&'z'].clone();
    println!("{:?}", search2(expr, Vec::new()));
    0
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc::str_input;
    const TEST_INPUT: &'static str = "\
";
    #[test]
    fn sample_a() {
        assert_eq!(solve_a(parse(str_input(TEST_INPUT))), 0)
    }
    #[test]
    fn sample_b() {
        assert_eq!(solve_b(parse(str_input(TEST_INPUT))), 0)
    }
}
