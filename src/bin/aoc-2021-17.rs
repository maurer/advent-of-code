fn incl(x: isize, range: (isize, isize)) -> bool {
    (range.0 <= x) && (range.1 >= x)
}
fn check_target(mut xvel: isize, mut yvel: isize, xrange: (isize, isize), yrange: (isize, isize)) -> Option<isize> {
    let mut x = 0;
    let mut y = 0;
    let mut ymax = 0;
    loop {
        if x > xrange.1 && xvel >= 0 {
            return None;
        }
        if x < xrange.0 && xvel <= 0 {
            return None;
        }
        if yvel < 0 && y < yrange.0 {
            return None;
        }
        x += xvel;
        y += yvel;
        if y > ymax {
            ymax = y;
        }
        if xvel > 0 {
            xvel -= 1;
        } else if xvel < 0 {
            xvel += 1;
        }
        yvel -= 1;
        if incl(x, xrange) && incl(y, yrange) {
            return Some(ymax);
        }
    }
}
fn solve_a(xrange: (isize, isize), yrange: (isize, isize)) -> isize {
    let mut ymm = 0;
    for yvel in 0..1000 {
        println!("yvel {}", yvel);
        for xvel in -1000..1000 {
            if let Some(ym) = check_target(xvel, yvel, xrange, yrange) {
                if ym > ymm {
                    ymm = ym;
                }
            }
        }
    }
    ymm
}
fn solve_b(xrange: (isize, isize), yrange: (isize, isize)) -> isize {
    let mut out = 0;
    for yvel in -1000..1000 {
        for xvel in -1000..1000 {
            if let Some(ym) = check_target(xvel, yvel, xrange, yrange) {
                println!("{}, {}", xvel, yvel);
                out += 1;
            }
        }
    }
    out
}

fn main() {
    println!("{}", solve_a((253, 280), (-73, -46)));
    println!("{}", solve_b((253, 280), (-73, -46)));
}

#[test]
fn sample_a() {
    assert_eq!(solve_a((20, 30), (-10, -5)), 45);
}
#[test]
fn sample_b() {
    assert_eq!(solve_b((20, 30), (-10, -5)), 112);
}
