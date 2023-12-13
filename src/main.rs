mod d1;
mod d10;
mod d11;
mod d12;
mod d13;
mod d14;
mod d15;
mod d16;
mod d17;
mod d18;
mod d19;
mod d2;
mod d20;
mod d21;
mod d22;
mod d23;
mod d24;
mod d25;
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
use crate::d12::Solver12;
use crate::d13::Solver13;
use crate::d14::Solver14;
use crate::d15::Solver15;
use crate::d16::Solver16;
use crate::d17::Solver17;
use crate::d18::Solver18;
use crate::d19::Solver19;
use crate::d2::Solver2;
use crate::d20::Solver20;
use crate::d21::Solver21;
use crate::d22::Solver22;
use crate::d23::Solver23;
use crate::d24::Solver24;
use crate::d25::Solver25;
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

    let mut threads = Vec::new();

    for arg in args {
        let day = arg.parse::<u32>().unwrap();
        let t = std::thread::spawn(move || {
            let soln = get_solver_from_day(day).solve();
            println!("{}: {}", day, soln);
        });
        threads.push(t);
    }
    for t in threads {
        t.join().unwrap();
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
        12 => Box::new(Solver12 {}),
        13 => Box::new(Solver13 {}),
        14 => Box::new(Solver14 {}),
        15 => Box::new(Solver15 {}),
        16 => Box::new(Solver16 {}),
        17 => Box::new(Solver17 {}),
        18 => Box::new(Solver18 {}),
        19 => Box::new(Solver19 {}),
        20 => Box::new(Solver20 {}),
        21 => Box::new(Solver21 {}),
        22 => Box::new(Solver22 {}),
        23 => Box::new(Solver23 {}),
        24 => Box::new(Solver24 {}),
        25 => Box::new(Solver25 {}),
        _ => panic!("Unknown solver"),
    }
}
