mod d1;
mod d2;
mod d3;

use crate::d1::Solver1;
use crate::d2::Solver2;
use crate::d3::Solver3;
use std::{env, str::Lines};

trait Solver {
    fn day_number(&self) -> u32;

    fn solve(&self) -> String {
        let input_text =
            std::fs::read_to_string(format!("src/d{}/input.txt", self.day_number())).unwrap();
        let lines = input_text.lines();

        let part1_soln = self.part1(lines.clone());
        let part2_soln = self.part2(lines);

        format!("part 1: {}, part 2: {}", part1_soln, part2_soln)
    }

    fn part1(&self, lines: Lines) -> String;
    fn part2(&self, lines: Lines) -> String;
}

fn main() {
    let args = env::args().skip(1); // Skip the executable name

    for arg in args {
        let day = arg.parse::<u32>().unwrap();
        let soln = get_solver_from_day(day).solve();
        println!("day {}: {}", day, soln);
    }
}

fn get_solver_from_day(day: u32) -> Box<dyn Solver> {
    match day {
        1 => Box::new(Solver1 {}),
        2 => Box::new(Solver2 {}),
        3 => Box::new(Solver3 {}),
        _ => panic!("Unknown solver"),
    }
}
