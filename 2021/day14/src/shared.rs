use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Rule<'a> {
    pub from: &'a str,
    pub insert: &'a str,
}

impl<'a> Rule<'a> {
    pub fn new(from: &'a str, insert: &'a str) -> Rule<'a> {
        Rule { from, insert }
    }
}

pub fn parse(input: &str) -> (&str, Vec<Rule>) {
    let mut lines = input.lines();

    let poly = lines.next().unwrap();
    lines.next();

    let mut rules: Vec<Rule> = vec![];
    for line in lines {
        let parts = line.split_once(" -> ");
        let rule = Rule {
            from: parts.unwrap().0,
            insert: parts.unwrap().1,
        };
        rules.push(rule);
    }

    (poly, rules)
}

type RuleMap = HashMap<(char, char), char>;

pub fn parse2(input: &str) -> (&str, RuleMap) {
    let mut lines = input.lines();

    let poly = lines.next().unwrap();
    lines.next();

    let mut rule_map: RuleMap = HashMap::new();
    for line in lines {
        let parts = line.split_once(" -> ").unwrap();
        let (from, to) = (parts.0, parts.1);
        let mut from_chars = from.chars();
        let c1 = from_chars.next().unwrap();
        let c2 = from_chars.next().unwrap();
        rule_map.insert((c1, c2), to.chars().next().unwrap());
    }
    (poly, rule_map)
}
