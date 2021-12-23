#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Range {
    low: isize,
    high: isize
}
impl Range {
    fn clampe(&mut self, bound: isize) {
        if self.low < -bound {
            self.low = -bound;
        }
        if self.high > bound {
            self.high = bound;
        }
    }
    fn size(&self) -> isize {
        1 + self.high - self.low
    }
    fn contains(&self, other: &Self) -> bool {
        self.low <= other.low && self.high >= other.high
    }
    fn inside(&self, k: isize) -> bool {
        self.low <= k && k <= self.high
    }

    fn ieq(&self, k: isize) -> bool {
        self.low <= k && k <= self.high
    }
 
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
enum Dir {
    On,
    Off
}
impl Dir {
    fn from_str(x: &str) -> Self {
        use Dir::*;
        match x {
            "on" => On,
            "off" => Off,
            _ => panic!("bad dir: {}", x),
        }
    }
}
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Insn {
    dir: Dir,
    x: Range,
    y: Range,
    z: Range
}
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Cuboid {
    x: Range,
    y: Range,
    z: Range,
}
impl Cuboid {
    fn volume(&self) -> isize {
        self.x.size() * self.y.size() * self.z.size()
    }
    fn ieq(&self, pt: (isize, isize, isize)) -> bool {
        self.x.ieq(pt.0) && self.y.ieq(pt.1) && self.z.ieq(pt.2)
    }
    fn shatter(&self, other: &Self) -> Vec<Self> {
        let mut x_s = Vec::new();
        x_s.push(self.x.low);
        if self.x.inside(other.x.low) {
            x_s.push(other.x.low);
        }
        if self.x.inside(other.x.high) {
            x_s.push(other.x.high + 1);
        }
        x_s.push(self.x.high + 1);

        let mut y_s = Vec::new();
        y_s.push(self.y.low);
        if self.y.inside(other.y.low) {
            y_s.push(other.y.low);
        }
        if self.y.inside(other.y.high) {
            y_s.push(other.y.high + 1);
        }
        y_s.push(self.y.high + 1);

        let mut z_s = Vec::new();
        z_s.push(self.z.low);
        if self.z.inside(other.z.low) {
            z_s.push(other.z.low);
        }
        if self.z.inside(other.z.high) {
            z_s.push(other.z.high + 1);
        }
        z_s.push(self.z.high + 1);

        let mut out = Vec::new();
        for (xl, xh) in x_s.iter().tuple_windows() {
            let x = Range {low: *xl, high: *xh - 1};
            for (yl, yh) in y_s.iter().tuple_windows() {
                let y = Range {low: *yl, high: *yh - 1};
                for (zl, zh) in z_s.iter().tuple_windows() {
                    let z = Range {low: *zl, high: *zh - 1};
                    out.push(Cuboid {x, y, z});
                }
            }
        }
        out
    }
    fn contains(&self, other: &Self) -> bool {
        self.x.contains(&other.x) && self.y.contains(&other.y) && self.z.contains(&other.z)
    }
}

fn repair(mut state: Vec<Cuboid>) -> Vec<Cuboid> {
    let mut cur = state.pop().unwrap();
    let mut out = Vec::new();
    while let Some(n) = state.pop() {
        let other = n;
        if cur.x == n.x && cur.y == n.y {
            if cur.z.low == other.z.high + 1 {
                cur.z.low = other.z.low;
                continue;
            } else if cur.z.high + 1 == other.z.low {
                cur.z.high = other.z.high;
                continue;
            }
        } else if cur.y == n.y && cur.z == n.z {
            if cur.x.low == other.x.high + 1 {
                cur.x.low = other.x.low;
                continue;
            } else if cur.x.high + 1== other.x.low {
                cur.x.high = other.x.high;
                continue;
            }
        } else if cur.x == n.x && cur.z == n.z {
            if cur.y.low == other.y.high + 1 {
                cur.y.low = other.y.low;
                continue;
            } else if 1 + cur.y.high == other.y.low {
                cur.y.high = other.y.high;
                continue;
            }
        } else {
            println!("Did not merge");
            out.push(cur);
            cur = n
        }
    }
    out.push(cur);
    out
}
type Input = Vec<Insn>;
fn parse(mut input: impl Iterator<Item = String>) -> Input {
    input.map(|s| {
        let (ds, xl, xh, yl, yh, zl, zh) = scanf!(s, "{} x={}..{},y={}..{},z={}..{}", String, isize, isize,isize, isize,isize, isize).unwrap();
        Insn {
            dir: Dir::from_str(&ds),
            x: Range {low: xl, high: xh},
            y: Range {low: yl, high: yh},
            z: Range {low: zl, high: zh},
        }
    }).collect_vec()

}
use sscanf::scanf;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
const INPUT: &str = include_str!("../../inputs/2021/22");
fn main() {
    println!(
        "A: {}\tB: {}",
        solve_a(parse(aoc::str_input(INPUT))),
        solve_b(parse(aoc::str_input(INPUT)))
    );
}
fn solve_a(mut input: Input) -> usize {
    return 0;
    let mut state = HashSet::new();
    for mut insn in input {
        insn.x.clampe(50);
        insn.y.clampe(50);
        insn.z.clampe(50);
        let mut switch = HashSet::new();
        for x in insn.x.low..(insn.x.high + 1) {
            for y in insn.y.low..(insn.y.high + 1) {
                for z in insn.z.low..(insn.z.high + 1) {
                    switch.insert((x, y, z));
                }
            }
        }
        match insn.dir {
            Dir::On => {
                state = &state | &switch;
            },
            Dir::Off => {
                state = state.difference(&switch).copied().collect();
            }
        }
    }
    state.len()
}
fn purge(state: Vec<Cuboid>, cuboid: Cuboid) -> Vec<Cuboid> {
    let mut shattered = Vec::new();
    for whole in state.into_iter() {
        let mut diced = whole.shatter(&cuboid);
        shattered.append(&mut diced);
    }
    shattered.into_iter().filter(|x| {
        !cuboid.contains(x)
    }).collect_vec()
}

fn solve_b(mut input: Input) -> isize {
    let mut state: Vec<Cuboid> = Vec::new();
    for (n, insn) in input.into_iter().enumerate() {
        println!("insn {}, state size {}", n, state.len());
        let cuboid = Cuboid {x: insn.x, y: insn.y, z: insn.z};
        state = purge(state, cuboid);
        match insn.dir {
            Dir::On => {
                state.push(cuboid);
            }
            _ => ()
        }
        //println!("{:?}", state);
        state = repair(state);
    }
    state.into_iter().map(|cub| cub.volume()).sum()
}
fn solve_a2(mut input: Input) -> isize {
    let mut state: Vec<Cuboid> = Vec::new();
    for (n, insn) in input.into_iter().enumerate() {
        println!("insn {}, state size {}", n, state.len());
        let cuboid = Cuboid {x: insn.x, y: insn.y, z: insn.z};
        state = purge(state, cuboid);
        match insn.dir {
            Dir::On => {
                state.push(cuboid);
            }
            _ => ()
        }
        println!("Pre-repair\n{:?}", state);
        println!("pre-repair volume {:?}", state.iter().map(|cub| cub.volume()).sum::<isize>());

        state = repair(state);
        println!("{:?}", state);
        println!("current volume {:?}", state.iter().map(|cub| cub.volume()).sum::<isize>());
    }
    let mut out: isize = 0;
    for x in -50..51 {
        for y in -50..51 {
            for z in -50..51 {
                if state.iter().any(|cub| cub.ieq((x, y, z))) {
                    out += 1;
                }
            }
        }
    }
    out
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc::str_input;
    const TEST_INPUT: &'static str = "\
on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682";
static TEST_TWO: &'static str = "\
on x=-5..47,y=-31..22,z=-19..33
on x=-44..5,y=-27..21,z=-14..35
on x=-49..-1,y=-11..42,z=-10..38
on x=-20..34,y=-40..6,z=-44..1
off x=26..39,y=40..50,z=-2..11
on x=-41..5,y=-41..6,z=-36..8
off x=-43..-33,y=-45..-28,z=7..25
on x=-33..15,y=-32..19,z=-34..11
off x=35..47,y=-46..-34,z=-11..5
on x=-14..36,y=-6..44,z=-16..29
on x=-57795..-6158,y=29564..72030,z=20435..90618
on x=36731..105352,y=-21140..28532,z=16094..90401
on x=30999..107136,y=-53464..15513,z=8553..71215
on x=13528..83982,y=-99403..-27377,z=-24141..23996
on x=-72682..-12347,y=18159..111354,z=7391..80950
on x=-1060..80757,y=-65301..-20884,z=-103788..-16709
on x=-83015..-9461,y=-72160..-8347,z=-81239..-26856
on x=-52752..22273,y=-49450..9096,z=54442..119054
on x=-29982..40483,y=-108474..-28371,z=-24328..38471
on x=-4958..62750,y=40422..118853,z=-7672..65583
on x=55694..108686,y=-43367..46958,z=-26781..48729
on x=-98497..-18186,y=-63569..3412,z=1232..88485
on x=-726..56291,y=-62629..13224,z=18033..85226
on x=-110886..-34664,y=-81338..-8658,z=8914..63723
on x=-55829..24974,y=-16897..54165,z=-121762..-28058
on x=-65152..-11147,y=22489..91432,z=-58782..1780
on x=-120100..-32970,y=-46592..27473,z=-11695..61039
on x=-18631..37533,y=-124565..-50804,z=-35667..28308
on x=-57817..18248,y=49321..117703,z=5745..55881
on x=14781..98692,y=-1341..70827,z=15753..70151
on x=-34419..55919,y=-19626..40991,z=39015..114138
on x=-60785..11593,y=-56135..2999,z=-95368..-26915
on x=-32178..58085,y=17647..101866,z=-91405..-8878
on x=-53655..12091,y=50097..105568,z=-75335..-4862
on x=-111166..-40997,y=-71714..2688,z=5609..50954
on x=-16602..70118,y=-98693..-44401,z=5197..76897
on x=16383..101554,y=4615..83635,z=-44907..18747
off x=-95822..-15171,y=-19987..48940,z=10804..104439
on x=-89813..-14614,y=16069..88491,z=-3297..45228
on x=41075..99376,y=-20427..49978,z=-52012..13762
on x=-21330..50085,y=-17944..62733,z=-112280..-30197
on x=-16478..35915,y=36008..118594,z=-7885..47086
off x=-98156..-27851,y=-49952..43171,z=-99005..-8456
off x=2032..69770,y=-71013..4824,z=7471..94418
on x=43670..120875,y=-42068..12382,z=-24787..38892
off x=37514..111226,y=-45862..25743,z=-16714..54663
off x=25699..97951,y=-30668..59918,z=-15349..69697
off x=-44271..17935,y=-9516..60759,z=49131..112598
on x=-61695..-5813,y=40978..94975,z=8655..80240
off x=-101086..-9439,y=-7088..67543,z=33935..83858
off x=18020..114017,y=-48931..32606,z=21474..89843
off x=-77139..10506,y=-89994..-18797,z=-80..59318
off x=8476..79288,y=-75520..11602,z=-96624..-24783
on x=-47488..-1262,y=24338..100707,z=16292..72967
off x=-84341..13987,y=2429..92914,z=-90671..-1318
off x=-37810..49457,y=-71013..-7894,z=-105357..-13188
off x=-27365..46395,y=31009..98017,z=15428..76570
off x=-70369..-16548,y=22648..78696,z=-1892..86821
on x=-53470..21291,y=-120233..-33476,z=-44150..38147
off x=-93533..-4276,y=-16170..68771,z=-104985..-24507";
//    #[test]
//    fn sample_a() {
//        assert_eq!(solve_a2(parse(str_input(TEST_INPUT))), 0)
//    }
//    #[test]
//    fn sample_b() {
//        assert_eq!(solve_b(parse(str_input(TEST_TWO))), 0)
//    }
//static SMALL: &'static str = "\
//on x=10..12,y=10..12,z=10..12
//on x=11..13,y=11..13,z=11..13
//off x=9..11,y=9..11,z=9..11
//on x=10..10,y=10..10,z=10..10";
//    #[test]
//    fn small() {
//        assert_eq!(solve_a2(parse(str_input(SMALL))), 39);
//    }
}
