use std::{cell::RefCell, collections::HashMap, rc::Rc, str::Lines};

use crate::Solver;
pub struct Solver15;

// struct ListNode {
//     lens: Lens,
//     next: Option<Rc<RefCell<ListNode>>>,
//     prev: Option<Rc<RefCell<ListNode>>>,
// }

struct LensBox {
    lens_map: HashMap<String, Rc<RefCell<Lens>>>,
    ordered_lenses: Vec<Rc<RefCell<Lens>>>,
}

impl LensBox {
    fn new() -> Self {
        Self {
            lens_map: HashMap::new(),
            ordered_lenses: Vec::new(),
        }
    }
}

enum LensBoxAction {
    Insert(Lens),
    Remove,
}
struct Lens {
    name: String,
    power: Option<usize>,
}

impl Solver for Solver15 {
    fn day_number(&self) -> u32 {
        15
    }

    fn part1(&self, mut input_lines: Lines) -> String {
        let input = input_lines.next().unwrap();
        let sum = input
            .split(',')
            .map(|s| {
                s.chars()
                    .fold(0, |acc, c| ((acc + c as u32 % 256) * 17) % 256)
            })
            .sum::<u32>();

        sum.to_string()
    }

    fn part2(&self, mut input_lines: Lines) -> String {
        let input = input_lines.next().unwrap();

        let mut lens_boxes: Vec<LensBox> = vec![];
        for _ in 0..256 {
            lens_boxes.push(LensBox::new());
        }

        for instruction in input.split(',') {
            let mut parts = instruction.split(|c| c == '=' || c == '-');
            let label = parts.next().unwrap().to_string();
            let box_ix = label
                .chars()
                .fold(0, |acc, c| ((acc + c as u32 % 256) * 17) % 256)
                as usize;
            let action = match parts.next().unwrap().parse::<usize>().ok() {
                Some(val) => LensBoxAction::Insert(Lens {
                    name: label.to_string(),
                    power: Some(val),
                }),
                None => LensBoxAction::Remove,
            };

            let target_box = &mut lens_boxes[box_ix];

            // Optimized over matching (action, lens_map.<some operation>)
            // Each possible branch only has a single hashmap operation,
            // except for the replace case, which cannot be done with one:
            // because we don't know whether to insert or replace until we've inserted
            // at which point we'd need to either re-insert the old value,
            // or somehow handle making the inserted value point to the right
            // lens in the vector.
            match action {
                LensBoxAction::Insert(new_lens) => {
                    match target_box.lens_map.get(&label) {
                        Some(old_key) => old_key.borrow_mut().power = new_lens.power,
                        None => {
                            let lens = Rc::new(RefCell::new(new_lens));
                            target_box
                                .lens_map
                                .insert(lens.borrow().name.clone(), lens.clone());
                            target_box.ordered_lenses.push(lens);
                        }
                    };
                }
                LensBoxAction::Remove => {
                    if let Some(old_lens) = target_box.lens_map.get(&label) {
                        old_lens.borrow_mut().power = None;
                    }
                }
            };
        }

        let focusing_power = lens_boxes
            .iter()
            .enumerate()
            .map(|(box_ix, lens_box)| {
                lens_box
                    .ordered_lenses
                    .iter()
                    .filter(|lens| lens.borrow().power.is_some())
                    .enumerate()
                    .map(move |(lens_ix, lens)| {
                        (1 + box_ix) * (1 + lens_ix) * lens.borrow().power.unwrap()
                    })
            })
            .flatten()
            .sum::<usize>();

        focusing_power.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solver;
    #[test]
    fn part1() {
        let sample_input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(super::Solver15.part1(sample_input.lines()), "1320");
    }

    #[test]
    fn part2() {
        let sample_input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(super::Solver15.part2(sample_input.lines()), "145");
    }
}
