use core::panic;
use std::{collections::HashMap, str::Lines};

use itertools::Itertools;

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
    x: Range,
    m: Range,
    a: Range,
    s: Range,
}

#[derive(Debug, Clone, Copy)]
struct Range {
    min: usize,
    max: usize,
}

fn attr_x(val: &mut Part) -> &mut Range {
    &mut val.x
}

fn attr_m(val: &mut Part) -> &mut Range {
    &mut val.m
}

fn attr_a(val: &mut Part) -> &mut Range {
    &mut val.a
}

fn attr_s(val: &mut Part) -> &mut Range {
    &mut val.s
}

fn build_compare_func(
    rule: Vec<char>,
    // comparitor: for<'a, 'b> fn(&'a usize, &'b usize) -> bool,
) -> Box<dyn Fn(Part) -> Vec<(Part, RuleResult)>> {
    let rulestring = rule[2..].iter().collect::<String>();
    let (cmp_val, dest) = rulestring.split_once(':').unwrap();
    let dest = dest.to_string();
    let cmp_val = cmp_val.parse::<usize>().unwrap();

    let attribute = match rule[0] {
        'x' => Box::new(&attr_x) as Box<dyn Fn(&mut Part) -> &mut Range>,
        'm' => Box::new(&attr_m) as Box<dyn Fn(&mut Part) -> &mut Range>,
        'a' => Box::new(&attr_a) as Box<dyn Fn(&mut Part) -> &mut Range>,
        's' => Box::new(&attr_s) as Box<dyn Fn(&mut Part) -> &mut Range>,
        _ => {
            panic!("Unknown attribute {}", rule[0])
        }
    };

    let action = match { dest.as_str() } {
        "R" => RuleResult::Reject,
        "A" => RuleResult::Accept,
        _ => RuleResult::NewRule(dest.clone()),
    };

    match rule[1] {
        '>' => Box::new(move |mut val| {
            if cmp_val >= attribute(&mut val).max {
                vec![(val, RuleResult::Pass)]
            } else if cmp_val < attribute(&mut val).min {
                vec![(val, action.clone())]
            } else {
                // split the range
                let mut passing = val.clone();
                let mut failing = val.clone();
                attribute(&mut passing).min = cmp_val + 1;
                attribute(&mut failing).max = cmp_val;
                vec![(passing, action.clone()), (failing, RuleResult::Pass)]
            }
        }),
        '<' => Box::new(move |mut val| {
            if cmp_val <= attribute(&mut val).min {
                vec![(val, RuleResult::Pass)]
            } else if cmp_val > attribute(&mut val).max {
                vec![(val, action.clone())]
            } else {
                // split the range
                let mut passing = val.clone();
                let mut failing = val.clone();
                attribute(&mut passing).max = cmp_val - 1;
                attribute(&mut failing).min = cmp_val;
                vec![(passing, action.clone()), (failing, RuleResult::Pass)]
            }
        }),

        _ => {
            panic!("Unknown comparitor {}", rule[1])
        }
    }
}

fn build_basic_func(res: RuleResult) -> Box<dyn Fn(Part) -> Vec<(Part, RuleResult)>> {
    Box::new(move |p| vec![(p, res.clone())])
}

impl Solver for Solver19 {
    fn day_number(&self) -> u32 {
        19
    }

    fn part1(&self, mut input_lines: Lines) -> String {
        let rules = build_rules(&mut input_lines);

        let parts = input_lines
            .map(|line| {
                let attributes = line.split(',').map(|part| {
                    let (_, val) = part
                        .trim_matches(|c| c == '{' || c == '}')
                        .split_once('=')
                        .unwrap();
                    val.parse::<usize>().unwrap()
                });
                let (x, m, a, s) = attributes.collect_tuple().unwrap();
                let part = Part {
                    x: Range { min: x, max: x },
                    m: Range { min: m, max: m },
                    a: Range { min: a, max: a },
                    s: Range { min: s, max: s },
                };

                (part, "in".to_string())
            })
            .collect_vec();

        let accepted_ranges = apply_rules(rules, parts);
        accepted_ranges
            .iter()
            .map(|p| (p.x.max + p.m.max + p.a.max + p.s.max))
            .sum::<usize>()
            .to_string()
    }

    fn part2(&self, mut input_lines: Lines) -> String {
        let rules = build_rules(&mut input_lines);

        let parts = vec![(
            Part {
                x: Range { min: 1, max: 4000 },
                m: Range { min: 1, max: 4000 },
                a: Range { min: 1, max: 4000 },
                s: Range { min: 1, max: 4000 },
            },
            "in".to_string(),
        )];

        let accepted_ranges = apply_rules(rules, parts);
        accepted_ranges
            .iter()
            .map(|p| {
                (p.x.max - p.x.min + 1) as u64
                    * (p.m.max - p.m.min + 1) as u64
                    * (p.a.max - p.a.min + 1) as u64
                    * (p.s.max - p.s.min + 1) as u64
            })
            .sum::<u64>()
            .to_string()
    }
}

fn apply_rules(
    rules: HashMap<String, Vec<Box<dyn Fn(Part) -> Vec<(Part, RuleResult)>>>>,
    mut parts: Vec<(Part, String)>,
) -> Vec<Part> {
    let mut accepted_parts = vec![];

    while let Some((mut part, rule_name)) = parts.pop() {
        // match action {
        // RuleResult::NewRule(rule) => {
        let workflow = rules.get(&rule_name).unwrap();
        for rule in workflow {
            let rule_output = rule(part);
            if let Some(p) = rule_output
                .into_iter()
                .filter_map(|(part, action)| match action {
                    RuleResult::Pass => Some(part),
                    RuleResult::Accept => {
                        accepted_parts.push(part);
                        None
                    }
                    RuleResult::NewRule(r) => {
                        parts.push((part, r));
                        None
                    }
                    RuleResult::Reject => None,
                })
                .collect::<Vec<Part>>()
                .get(0)
            {
                part = *p;
            } else {
                break;
            }
        }
    }
    // _ => panic!("Unexpected action {:?} for part {:?}", action, part),
    // }

    accepted_parts
}

fn build_rules(
    input_lines: &mut Lines,
) -> HashMap<String, Vec<Box<dyn Fn(Part) -> Vec<(Part, RuleResult)>>>> {
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
                'R' => build_basic_func(RuleResult::Reject),
                'A' => build_basic_func(RuleResult::Accept),
                _ => match rule[1] {
                    '>' | '<' => build_compare_func(rule),
                    _ => Box::new(move |p| {
                        vec![(p, RuleResult::NewRule(rule.iter().collect::<String>()))]
                    }) as Box<dyn Fn(_) -> Vec<(Part, RuleResult)>>,
                },
            };
            workflow_vec.push(rule);
        }

        rules.insert(key.to_string(), workflow_vec);
    }
    rules
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
