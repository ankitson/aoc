use itertools::Itertools;
use regex::Regex;
use z3::ast::*;
use z3::*;

pub type P3 = (isize, isize, isize);
pub type Input = Vec<(P3, P3)>;
pub type Output = usize;

pub fn parse(input: &str) -> Vec<(P3, P3)> {
    input
        .lines()
        .map(|line| {
            let (raw_pos, raw_vel) = line.split_once(" @ ").unwrap();
            let (px, py, pz) = raw_pos.split(",").map(|s| s.trim().parse::<isize>().unwrap()).collect_tuple().unwrap();
            let (vx, vy, vz) = raw_vel.split(",").map(|s| s.trim().parse::<isize>().unwrap()).collect_tuple().unwrap();
            ((px, py, pz), (vx, vy, vz))
        })
        .collect_vec()
}

pub fn part1(raw_input: &str, bounds: (P3, P3)) -> Output {
    let stones = parse(raw_input);
    let mut count = 0;
    for comb in stones.iter().combinations(2) {
        let (((p1x, p1y, _), (v1x, v1y, _)), ((p2x, p2y, _), (v2x, v2y, _))) = (comb[0], comb[1]);
        let m1 = *v1y as f64 / *v1x as f64;
        let m2 = *v2y as f64 / *v2x as f64;
        let c1 = *p1y as f64 - m1 * *p1x as f64;
        let c2 = *p2y as f64 - m2 * *p2x as f64;

        if m1 == m2 {
            continue;
        }
        let x_intsct = ((c2 - c1) / (m1 - m2)) as isize;
        let y_instct = (m1 * x_intsct as f64 + c1) as isize;

        let in_bounds =
            x_intsct >= bounds.0 .0 && x_intsct <= bounds.1 .0 && y_instct >= bounds.0 .1 && y_instct <= bounds.1 .1;
        let direction = (*v1x >= 0 && x_intsct >= *p1x || *v1x < 0 && x_intsct <= *p1x)
            && (*v2x >= 0 && x_intsct >= *p2x || *v2x < 0 && x_intsct <= *p2x);
        if in_bounds && direction {
            count += 1;
        }
    }
    count
}

pub fn z3_experiment() {
    // Create a Z3 context
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    // Create a solver
    let solver = Solver::new(&ctx);

    // Create a variable x
    let x = ast::Int::new_const(&ctx, "x");

    // Add constraint: x + 2 = 5
    let two = ast::Int::from_i64(&ctx, 2);
    let five = ast::Int::from_i64(&ctx, 5);
    // First create the left side of the equation (x + 2)
    let left_side = &x + &two;
    // Then create the equality comparison
    let equation = left_side._eq(&five);

    solver.assert(&equation);

    // Check if the equation is satisfiable
    match solver.check() {
        z3::SatResult::Sat => {
            // Get the model
            let model = solver.get_model().unwrap();
            println!("x = {}", model.eval(&x, true).unwrap());
        }
        z3::SatResult::Unsat => println!("No solution exists"),
        z3::SatResult::Unknown => println!("Failed to solve"),
    }

    solver.reset();

    // Create variables x and y
    let x = ast::Int::new_const(&ctx, "x");
    let y = ast::Int::new_const(&ctx, "y");

    // Add constraints:
    // x + y = 10
    // x > 0
    // y > 0
    let ten = ast::Int::from_i64(&ctx, 10);
    let zero = ast::Int::from_i64(&ctx, 0);
    let three = ast::Int::from_i64(&ctx, 3);

    solver.assert(&(&x + &y)._eq(&ten));
    solver.assert(&x.gt(&three));
    solver.assert(&y.gt(&zero));

    match solver.check() {
        z3::SatResult::Sat => {
            let model = solver.get_model().unwrap();
            println!("x = {}", model.eval(&x, true).unwrap());
            println!("y = {}", model.eval(&y, true).unwrap());
        }
        z3::SatResult::Unsat => println!("No solution exists"),
        z3::SatResult::Unknown => println!("Failed to solve"),
    }
}

pub fn part2(raw_input: &str) -> Output {
    let stones = parse(raw_input);
    let ctx = z3::Context::new(&z3::Config::new());
    let s = z3::Solver::new(&ctx);
    let [fx, fy, fz, fdx, fdy, fdz] = ["fx", "fy", "fz", "fdx", "fdy", "fdz"].map(|v| Real::new_const(&ctx, v));
    let zero = Int::from_i64(&ctx, 0).to_real();
    for (i, &((x, y, z), (dx, dy, dz))) in stones[..3].iter().enumerate() {
        let [x, y, z, dx, dy, dz] = [x, y, z, dx, dy, dz].map(|v| Int::from_i64(&ctx, v as _).to_real());
        let t = Real::new_const(&ctx, format!("t{i}"));
        s.assert(&t.ge(&zero));
        s.assert(&((&x + &dx * &t)._eq(&(&fx + &fdx * &t))));
        s.assert(&((&y + &dy * &t)._eq(&(&fy + &fdy * &t))));
        s.assert(&((&z + &dz * &t)._eq(&(&fz + &fdz * &t))));
    }
    assert_eq!(s.check(), z3::SatResult::Sat);
    let res = s.get_model().unwrap().eval(&(&fx + &fy + &fz), true).unwrap();
    res.to_string().strip_suffix(".0").unwrap().parse().unwrap()
}
