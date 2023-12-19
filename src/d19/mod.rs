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
    min: i64,
    max: i64,
}

impl Range {
    fn diff(&self) -> i64 {
        self.max + 1 - self.min
    }
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

fn build_compare_func(rule: Vec<char>) -> Box<dyn Fn(Part) -> Vec<(Part, RuleResult)>> {
    let rulestring = rule[2..].iter().collect::<String>();
    let (cmp_val, dest) = rulestring.split_once(':').unwrap();
    let dest = dest.to_string();
    let cmp_val = cmp_val.parse::<i64>().unwrap();

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

    let (split_offset, lower_action, higher_action) = match rule[1] {
        '<' => (0, action, RuleResult::Pass),
        '>' => (1, RuleResult::Pass, action),
        _ => {
            panic!("Unknown comparitor {}", rule[1])
        }
    };

    Box::new(move |mut range| {
        if attribute(&mut range).max < cmp_val + split_offset {
            // All values are lower
            vec![(range, lower_action.clone())]
        } else if attribute(&mut range).min > cmp_val + split_offset - 1 {
            // All values are higher
            vec![(range, higher_action.clone())]
        } else {
            // split the range
            let mut low_range = range;
            let mut high_range = range;
            attribute(&mut low_range).max = cmp_val + split_offset - 1;
            attribute(&mut high_range).min = cmp_val + split_offset;
            vec![
                (low_range, lower_action.clone()),
                (high_range, higher_action.clone()),
            ]
        }
    })
}

fn build_constant_func(res: RuleResult) -> Box<dyn Fn(Part) -> Vec<(Part, RuleResult)>> {
    Box::new(move |p| vec![(p, res.clone())])
}

fn apply_rules(
    rules: HashMap<String, Vec<Box<dyn Fn(Part) -> Vec<(Part, RuleResult)>>>>,
    mut parts: Vec<(Part, String)>,
) -> Vec<Part> {
    let mut accepted_parts = vec![];

    while let Some((mut part_for_workflow, workflow_name)) = parts.pop() {
        let workflow = rules.get(&workflow_name).unwrap();
        for rule in workflow {
            let rule_output = rule(part_for_workflow);
            if let Some(part_for_next_workflow) = rule_output
                .into_iter()
                .filter_map(|(p, action)| match action {
                    RuleResult::Pass => Some(p),
                    RuleResult::Accept => {
                        accepted_parts.push(p);
                        None
                    }
                    RuleResult::NewRule(r) => {
                        parts.push((p, r));
                        None
                    }
                    RuleResult::Reject => None,
                })
                .collect::<Vec<Part>>()
                .get(0)
            {
                part_for_workflow = *part_for_next_workflow;
            } else {
                break;
            }
        }
    }

    accepted_parts
}

fn build_rules(
    input_lines: &mut Lines,
) -> HashMap<String, Vec<Box<dyn Fn(Part) -> Vec<(Part, RuleResult)>>>> {
    let mut rules = HashMap::new();
    while let Some((key, workflow)) = input_lines.next().and_then(|l| l.split_once('{')) {
        let mut workflow_vec = vec![];

        for rule in workflow.trim_end_matches('}').split(',') {
            let rule: Vec<char> = rule.chars().collect();
            let rule = match rule[0] {
                'R' => build_constant_func(RuleResult::Reject),
                'A' => build_constant_func(RuleResult::Accept),
                _ => match rule[1] {
                    '>' | '<' => build_compare_func(rule),
                    _ => build_constant_func(RuleResult::NewRule(rule.iter().collect::<String>())),
                },
            };
            workflow_vec.push(rule);
        }

        rules.insert(key.to_string(), workflow_vec);
    }
    rules
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
                    val.parse::<i64>().unwrap()
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

        apply_rules(rules, parts)
            .iter()
            .map(|p| [p.x.max, p.m.max, p.a.max, p.s.max])
            .flatten()
            .sum::<i64>()
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

        apply_rules(rules, parts)
            .iter()
            .map(|p| {
                [p.x, p.m, p.a, p.s]
                    .iter()
                    .map(|r| r.diff())
                    .product::<i64>()
            })
            .sum::<i64>()
            .to_string()
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
