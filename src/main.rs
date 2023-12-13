mod d1;
mod d10;
mod d11;
mod d13;
mod d2;
mod d3;
mod d4;
mod d5;
mod d6;
mod d7;
mod d8;
mod d9;

use crate::d1::Solver1;
use crate::d10::Solver10;
use crate::d11::Solver11;
use crate::d13::Solver13;
use crate::d2::Solver2;
use crate::d3::Solver3;
use crate::d4::Solver4;
use crate::d5::Solver5;
use crate::d6::Solver6;
use crate::d7::Solver7;
use crate::d8::Solver8;
use crate::d9::Solver9;

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
        7 => Box::new(Solver7 {}),
        8 => Box::new(Solver8 {}),
        9 => Box::new(Solver9 {}),
        10 => Box::new(Solver10 {}),
        11 => Box::new(Solver11 {}),
        13 => Box::new(Solver13 {}),
        _ => panic!("Unknown solver"),
    }
}
