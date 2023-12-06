mod d1;
mod d2;
mod d3;
mod d4;
mod d5;
mod d6;

use crate::d1::Solver1;
use crate::d2::Solver2;
use crate::d3::Solver3;
use crate::d4::Solver4;
use crate::d5::Solver5;
use crate::d6::Solver6;

use std::{env, str::Lines};

trait Solver {
    fn day_number(&self) -> u32;

    fn solve(&self) -> String {
        let input_text =
            std::fs::read_to_string(format!("src/d{}/input.txt", self.day_number())).unwrap();
        let lines = input_text.lines();

        let part1_soln = self.part1(lines.clone());
        let part2_soln = self.part2(lines);

        format!("part 1: {},\tpart 2: {}", part1_soln, part2_soln)
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
        4 => Box::new(Solver4 {}),
        5 => Box::new(Solver5 {}),
        6 => Box::new(Solver6 {}),
        _ => panic!("Unknown solver"),
    }
}
