use itertools::Itertools;
use z3::{
    ast::{Ast, Int},
    Config, Context, SatResult, Solver,
};

fn process(input: &str) -> i64 {
    let config = Config::new();
    let ctx = Context::new(&config);
    let solver = Solver::new(&ctx);

    let x = Int::new_const(&ctx, "x");
    let y = Int::new_const(&ctx, "y");
    let z = Int::new_const(&ctx, "z");
    let vx = Int::new_const(&ctx, "vx");
    let vy = Int::new_const(&ctx, "vy");
    let vz = Int::new_const(&ctx, "vz");

    let stones = input
        .lines()
        .map(|line| {
            line.split(['@', ','])
                .map(|s| s.trim().parse::<i64>().unwrap())
                .collect_tuple::<(_, _, _, _, _, _)>()
                .unwrap()
        })
        .collect::<Vec<_>>();

    let range = 0..3;
    let t = range
        .clone()
        .map(|i| Int::new_const(&ctx, format!("t{}", i)))
        .collect::<Vec<_>>();

    for i in range {
        let sx = Int::from_i64(&ctx, stones[i].0);
        let sy = Int::from_i64(&ctx, stones[i].1);
        let sz = Int::from_i64(&ctx, stones[i].2);
        let svx = Int::from_i64(&ctx, stones[i].3);
        let svy = Int::from_i64(&ctx, stones[i].4);
        let svz = Int::from_i64(&ctx, stones[i].5);
        solver.assert(&(&x + &t[i] * &vx)._eq(&(sx + &t[i] * svx)));
        solver.assert(&(&y + &t[i] * &vy)._eq(&(sy + &t[i] * svy)));
        solver.assert(&(&z + &t[i] * &vz)._eq(&(sz + &t[i] * svz)));
    }

    assert_eq!(solver.check(), SatResult::Sat);

    let model = solver.get_model().unwrap();
    let x = model.eval(&x, true).unwrap().as_i64().unwrap();
    let y = model.eval(&y, true).unwrap().as_i64().unwrap();
    let z = model.eval(&z, true).unwrap().as_i64().unwrap();

    x + y + z
}

fn main() {
    let input = include_str!("../../input.txt");
    dbg!(process(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

    #[test]
    fn it_works() {
        assert_eq!(process(TEST), 47);
    }
}
