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
