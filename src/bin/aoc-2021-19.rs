use sscanf::scanf;
use bitvec::prelude::*;
type BV = BitVec<Msb0, u8>;
type BS = BitSlice<Msb0, u8>;
type Loc = (isize, isize, isize);
type Pos = Loc;
type Input = Vec<Vec<Loc>>;
fn parse(mut input: impl Iterator<Item = String>) -> Input {
    let mut out = Vec::new();
    let mut scanner = Vec::new();
    while let Some(line) = input.next() {
        if line.as_str() == "" {
            continue;
        }
        if line.as_str().starts_with("--- scanner") {
            if scanner.len() > 0 {
                out.push(scanner);
                scanner = Vec::new();
            }
            continue;
        }
        scanner.push(scanf!(line, "{},{},{}", isize, isize, isize).unwrap());
    }
    if scanner.len() > 0 {
        out.push(scanner);
    }
    out
}
fn rotate_x(p: Pos) -> Pos {
    (p.0, -p.2, p.1)
}
fn rotate_y(p: Pos) -> Pos {
    (-p.2, p.1, p.0)
}
fn rotate_z(p: Pos) -> Pos {
    (-p.1, p.0, p.2)
}

fn all_rotations(scan: &[Pos]) -> Vec<Vec<Pos>> {
    let mut out = Vec::new();
    let mut vec_x = scan.to_vec();
    // I think this can be 4 3 2 but this is probably safer (though it repeats rotations)
    for x in 0..4 {
        let mut vec_y = vec_x.clone();
        for y in 0..4 {
            let mut vec_z = vec_y.clone();
            for z in 0..4 {
                out.push(vec_z.clone());
                vec_z.iter_mut().for_each(|p| *p = rotate_z(*p));
            }
            vec_y.iter_mut().for_each(|p| *p = rotate_y(*p));
        }
        vec_x.iter_mut().for_each(|p| *p = rotate_x(*p));
    }
    out
}
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
const INPUT: &str = include_str!("../../inputs/2021/19");
fn main() {
    //println!(
    //    "A: {}\tB: {}",
    //    solve_a(parse(aoc::str_input(INPUT))),
    //    solve_b(parse(aoc::str_input(INPUT)))
    //);
    println!("{}", solve_b(parse(aoc::str_input(INPUT))));
}

fn manhattan(x: &Pos, y: &Pos) -> usize {
   ((x.0 - y.0).abs() + (x.1 - y.1).abs() + (x.2 - y.2).abs()) as usize
}

fn bestalign(x: &HashSet<Pos>, y: &[Pos]) -> Option<(Vec<Pos>, Pos)> {
    for xq in x.iter() {
        for yq in y {
            let off_x = yq.0 - xq.0;
            let off_y = yq.1 - xq.1;
            let off_z = yq.2 - xq.2;
            let mut align = 0;
            for xi in x.iter() {
                if y.contains(&(xi.0 + off_x, xi.1 + off_y, xi.2 + off_z)) {
                    align += 1;
                }
            }
            if align >= 12 {
                return Some(((y.iter().map(|p| (p.0 - off_x, p.1 - off_y, p.2 - off_z)).collect_vec()), (off_x, off_y, off_z)))
            }
        }
    }
    None
}

fn solve_a(mut input: Input) -> usize {
    let mut left = 0;
    let mut base: HashSet<Loc> = input.pop().unwrap().into_iter().collect();
    while input.len() != 0 {
        println!("Remaining to merge: {:?}", input.len());
        let mut nuke = 99999999;
        'big: for (idx, scan) in input.iter().enumerate() {
            for rot in all_rotations(&scan)  {
                if let Some((aligned,_)) = bestalign(&base, &rot) {
                    let ha: HashSet<Loc> = aligned.into_iter().collect();
                    base = &base | &ha;
                    nuke = idx;
                    break 'big;
                }
            }
        }
        input.remove(nuke);
    }
    base.len()
}
fn solve_b(mut input: Input) -> usize {
    let mut left = 0;
    let mut base: HashSet<Loc> = input.pop().unwrap().into_iter().collect();
    let mut offs = vec![(0, 0, 0)];
    while input.len() != 0 {
        println!("Remaining to merge: {:?}", input.len());
        let mut nuke = 99999999;
        'big: for (idx, scan) in input.iter().enumerate() {
            for rot in all_rotations(&scan)  {
                if let Some((aligned, off)) = bestalign(&base, &rot) {
                    offs.push(off);
                    let ha: HashSet<Loc> = aligned.into_iter().collect();
                    base = &base | &ha;
                    nuke = idx;
                    break 'big;
                }
            }
        }
        input.remove(nuke);
    }
    offs.into_iter().permutations(2).map(|p| manhattan(&p[0], &p[1])).max().unwrap()
}
#[cfg(test)]
mod test {
    use super::*;
    use aoc::str_input;
    const TEST_INPUT: &'static str = "\
--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14";
static ROT_INPUT: &'static str = "\
--- scanner 0 ---
-1,-1,1
-2,-2,2
-3,-3,3
-2,-3,1
5,6,-4
8,0,7";
   // #[test]
   // fn sample_a() {
   //     assert_eq!(solve_a(parse(str_input(TEST_INPUT))), 79)
   // }
   // #[test]
   // fn sample_b() {
   //     assert_eq!(solve_b(parse(str_input(TEST_INPUT))), 3621)
   //}
    #[test]
    fn sad() {
        let mut rots = all_rotations(parse(str_input(ROT_INPUT))[0].as_slice()).into_iter().map(|x| x[4]).collect_vec();
        rots.sort();
        rots.dedup();
        println!("{:?}", rots);
        println!("{:?}", rots.len());
    }
    #[test]
    fn rotate() {
        let x = (1, 2, 3);
        let mut y = x;
        for _ in 0..4 {
            y = rotate_x(y);
        }
        assert_eq!(x, y);
        for _ in 0..4 {
            y = rotate_y(y);
        }
        assert_eq!(x, y);
        for _ in 0..4 {
            y = rotate_z(y);
        }
        assert_eq!(x, y);

    }
}
