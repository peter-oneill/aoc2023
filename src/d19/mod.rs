use core::panic;
use std::{collections::HashMap, str::Lines};

use crate::Solver;
pub struct Solver19;

#[derive(Debug, Clone)]
enum RuleResult {
    Accept,
    Reject,
    Pass,
    NewRule(String),
}
#[derive(Debug, Clone, Copy)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

fn accept<T>(_: T) -> RuleResult {
    RuleResult::Accept
}

fn reject<T>(_: T) -> RuleResult {
    RuleResult::Reject
}

fn build_compare_func(
    rule: Vec<char>,
    // attribute: char,
    comparitor: for<'a, 'b> fn(&'a usize, &'b usize) -> bool,
    // cmp_val: usize,
    // dest: String,
) -> Box<dyn Fn(Part) -> RuleResult> {
    let rulestring = rule[2..].iter().collect::<String>();
    let (cmp_val, dest) = rulestring.split_once(':').unwrap();
    let dest = dest.to_string();
    let cmp_val = cmp_val.parse::<usize>().unwrap();

    let attribute = match rule[0] {
        'x' => |val: Part| val.x,
        'm' => |val: Part| val.m,
        'a' => |val: Part| val.a,
        's' => |val: Part| val.s,
        _ => {
            panic!("Unknown attribute {}", rule[0])
        }
    };

    let action = match { dest.as_str() } {
        "R" => RuleResult::Reject,
        "A" => RuleResult::Accept,
        _ => RuleResult::NewRule(dest.clone()),
    };
    Box::new(move |val| {
        if comparitor(&attribute(val), &cmp_val) {
            action.clone()
        } else {
            RuleResult::Pass
        }
    }) as Box<dyn Fn(_) -> RuleResult>
}

impl Solver for Solver19 {
    fn day_number(&self) -> u32 {
        19
    }

    fn part1(&self, mut input_lines: Lines) -> String {
        let mut rules = HashMap::new();
        while let Some(line) = input_lines.next() {
            if line.is_empty() {
                break;
            }
            let (key, workflow) = line.split_once("{").unwrap();
            let mut workflow_vec = vec![];

            for rule in workflow.trim_end_matches('}').split(',') {
                let rule: Vec<char> = rule.chars().collect();
                let rule = match rule[0] {
                    'R' => Box::new(reject) as Box<dyn Fn(_) -> RuleResult>,
                    'A' => Box::new(accept) as Box<dyn Fn(_) -> RuleResult>,
                    _ => match rule[1] {
                        '>' => build_compare_func(rule, usize::gt),
                        '<' => build_compare_func(rule, usize::lt),
                        _ => {
                            Box::new(move |_| RuleResult::NewRule(rule.iter().collect::<String>()))
                        }
                    },
                };
                workflow_vec.push(rule);
            }

            rules.insert(key.to_string(), workflow_vec);
        }

        let mut parts = vec![];

        while let Some(line) = input_lines.next() {
            // Now handling values
            let mut attributes = line.split(',').map(|part| {
                let (_, val) = part
                    .trim_matches(|c| c == '{' || c == '}')
                    .split_once('=')
                    .unwrap();
                val.parse::<usize>().unwrap()
            });
            let part = Part {
                x: attributes.next().unwrap(),
                m: attributes.next().unwrap(),
                a: attributes.next().unwrap(),
                s: attributes.next().unwrap(),
            };

            // let mut result = RuleResult::Pass;
            parts.push(part);
        }

        parts
            .iter()
            .map(|part| apply_rules(&rules, part))
            .sum::<usize>()
            .to_string()
    }

    fn part2(&self, mut input_lines: Lines) -> String {
        let mut rules = HashMap::new();
        while let Some(line) = input_lines.next() {
            if line.is_empty() {
                break;
            }
            let (key, workflow) = line.split_once("{").unwrap();
            let mut workflow_vec = vec![];

            for rule in workflow.trim_end_matches('}').split(',') {
                let rule: Vec<char> = rule.chars().collect();
                let rule = match rule[0] {
                    'R' => Box::new(reject) as Box<dyn Fn(_) -> RuleResult>,
                    'A' => Box::new(accept) as Box<dyn Fn(_) -> RuleResult>,
                    _ => match rule[1] {
                        '>' => build_compare_func(rule, usize::gt),
                        '<' => build_compare_func(rule, usize::lt),
                        _ => {
                            Box::new(move |_| RuleResult::NewRule(rule.iter().collect::<String>()))
                        }
                    },
                };
                workflow_vec.push(rule);
            }

            rules.insert(key.to_string(), workflow_vec);
        }

        let mut parts = vec![];

        parts
            .iter()
            .map(|part| apply_rules(&rules, part))
            .sum::<usize>()
            .to_string()
    }
}

fn apply_rules(
    rules: &HashMap<String, Vec<Box<dyn Fn(Part) -> RuleResult>>>,
    part: &Part,
) -> usize {
    let mut current_rule = "in".to_string();

    loop {
        let workflow = rules.get(&current_rule).unwrap();

        let mut new_rule = None;
        for rule in workflow {
            match rule(*part) {
                RuleResult::Accept => {
                    return part.x + part.m + part.a + part.s;
                }
                RuleResult::Reject => {
                    return 0;
                }
                RuleResult::Pass => {}
                RuleResult::NewRule(rule) => {
                    new_rule = Some(rule);
                    break;
                }
            }
        }
        if let Some(rule) = new_rule {
            current_rule = rule;
        } else {
            return 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solver;
    #[test]
    fn part1() {
        let sample_input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
        assert_eq!(super::Solver19.part1(sample_input.lines()), "19114");
    }

    #[test]
    fn part2() {
        let sample_input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
        assert_eq!(
            super::Solver19.part2(sample_input.lines()),
            "167409079868000"
        );
    }
}
