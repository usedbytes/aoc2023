use std::env;
use std::error::Error;
use std::fs::File;
use std::collections::HashMap;
use std::io::{self, BufRead};
use std::str::FromStr;
use std::ops::Range;

#[derive(Debug, Clone)]
struct ParseErr;

impl Error for ParseErr {}

impl std::fmt::Display for ParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "parse error")
    }
}

#[derive(Debug, Clone)]
enum Category {
    X,
    M,
    A,
    S,
}

impl FromStr for Category {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => { Ok(Category::X) },
            "m" => { Ok(Category::M) },
            "a" => { Ok(Category::A) },
            "s" => { Ok(Category::S) },
            _ => Err(ParseErr{}),
        }
    }
}

#[derive(Debug, Clone)]
struct RangePart {
    x: Range<u32>,
    m: Range<u32>,
    a: Range<u32>,
    s: Range<u32>,
}

impl RangePart {
    fn is_empty(&self) -> bool {
        return self.x.is_empty() || self.m.is_empty() || self.a.is_empty() || self.s.is_empty();
    }

    fn overlap(&self, other: &RangePart) -> RangePart {
        return RangePart{
            x: Range{
                start: std::cmp::max(self.x.start, other.x.start),
                end: std::cmp::min(self.x.end, other.x.end),
            },
            m: Range{
                start: std::cmp::max(self.m.start, other.m.start),
                end: std::cmp::min(self.m.end, other.m.end),
            },
            a: Range{
                start: std::cmp::max(self.a.start, other.a.start),
                end: std::cmp::min(self.a.end, other.a.end),
            },
            s: Range{
                start: std::cmp::max(self.s.start, other.s.start),
                end: std::cmp::min(self.s.end, other.s.end),
            },
        }
    }

    fn count(&self) -> u64 {
        if self.is_empty() {
            return 0;
        }

        let mut p: u64 = 1;

        if !self.x.is_empty() {
            p *= (self.x.end - self.x.start) as u64;
        }
        if !self.m.is_empty() {
            p *= (self.m.end - self.m.start) as u64;
        }
        if !self.a.is_empty() {
            p *= (self.a.end - self.a.start) as u64;
        }
        if !self.s.is_empty() {
            p *= (self.s.end - self.s.start) as u64;
        }

        return p;
    }
}

#[derive(Debug, Clone)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Part {
    fn parse(s: &str) -> Part {
        let values: Vec<&str> = s[1..s.len()-1].split(",").collect();

        let mut part = Part{ x: 0, m: 0, a: 0, s: 0 };

        for value in values {
            let (name, num) = value.split_once("=").unwrap();
            let num = num.parse::<u32>().unwrap();
            match name {
                "x" => { part.x = num },
                "m" => { part.m = num },
                "a" => { part.a = num },
                "s" => { part.s = num },
                _ => panic!(),
            }
        }

        return part;
    }

    fn get(&self, category: Category) -> u32 {
        match category {
            Category::X => { return self.x },
            Category::M => { return self.m },
            Category::A => { return self.a },
            Category::S => { return self.s },
        }
    }

    fn value(&self) -> u32 {
        return self.x + self.m + self.a + self.s;
    }
}

#[derive(Debug, Clone)]
enum RuleType {
    Lt,
    Gt,
    Always,
}

#[derive(Debug, Clone)]
struct Rule {
    category: Option<Category>,
    value: u32,
    target: String,
    rule_type: RuleType,
}

impl Rule {
    fn check_lt(&self, part: &Part) -> bool {
        return part.get(self.category.as_ref().unwrap().clone()) < self.value;
    }

    fn check_gt(&self, part: &Part) -> bool {
        return part.get(self.category.as_ref().unwrap().clone()) > self.value;
    }

    fn check(&self, part: &Part) -> bool {
        match self.rule_type {
            RuleType::Lt => return self.check_lt(part),
            RuleType::Gt => return self.check_gt(part),
            RuleType::Always => return true,
        }
    }

    fn parse_cmp(cmp: &str, target: &str) -> Result<Self, <Rule as FromStr>::Err> {
        let cat = &cmp[0..1];
        let op = &cmp[1..2];
        let val = &cmp[2..].parse::<u32>().expect("bad number");

        let category = Category::from_str(cat)?;
        match op {
            "<" => {
                Ok(Rule{
                    category: Some(category),
                    value: *val,
                    target: target.to_string(),
                    rule_type: RuleType::Lt,
                })
            },
            ">" => {
                Ok(Rule{
                    category: Some(category),
                    value: *val,
                    target: target.to_string(),
                    rule_type: RuleType::Gt,
                })
            },
            _ => Err(ParseErr),
        }
    }

    fn check_range(&self, part: &RangePart) -> (RangePart, RangePart) {
        if let Some(category) = &self.category {
            let mut pass = part.clone();
            let mut fail = part.clone();

            let (pass_range, fail_range) = match category {
                Category::X => {
                    (&mut pass.x, &mut fail.x)
                },
                Category::M => {
                    (&mut pass.m, &mut fail.m)
                },
                Category::A => {
                    (&mut pass.a, &mut fail.a)
                },
                Category::S => {
                    (&mut pass.s, &mut fail.s)
                },
            };

            match self.rule_type {
                RuleType::Lt => {
                    pass_range.end = std::cmp::min(pass_range.end, self.value);
                    fail_range.start = std::cmp::max(fail_range.start, self.value);
                },
                RuleType::Gt => {
                    pass_range.start = std::cmp::max(pass_range.start, self.value + 1);
                    fail_range.end = std::cmp::min(fail_range.end, self.value + 1);
                },
                _ => unreachable!(),
            }

            return (pass, fail);
        }

        return (
            part.clone(),
            RangePart{
                x: Range{ start: 0, end: 0 },
                m: Range{ start: 0, end: 0 },
                a: Range{ start: 0, end: 0 },
                s: Range{ start: 0, end: 0 },
            }
        )
    }
}

impl FromStr for Rule {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(':') {
            Some((cmp, target)) => {
                Self::parse_cmp(cmp, target)
            },
            None => {
                Ok(Rule{
                    category: None,
                    value: 0,
                    target: s.to_string(),
                    rule_type: RuleType::Always,
                })
            },
        }
    }
}

#[derive(Debug, Clone)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl FromStr for Workflow {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, rest) = s.split_once("{").ok_or(ParseErr{})?;

        let rs: Vec<&str> = rest[0..rest.len()-1].split(",").collect();
        let rules = rs.iter().map(|s| Rule::from_str(s)).collect::<Result<Vec<_>, _>>()?;

        Ok(Workflow{
            name: name.to_string(),
            rules: rules,
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let fname = &args[1];

    let file = File::open(fname)?;
    let reader = io::BufReader::new(file);
    let mut lines = reader.lines();

    let mut workflows: HashMap<String, Workflow> = HashMap::new();

    while let Some(line) = lines.next() {
        let line = line?;

        if line.len() == 0 {
            break
        }

        let wf = Workflow::from_str(&line)?;
        workflows.insert(wf.name.to_string(), wf);
    }


    let mut total = 0;

    while let Some(line) = lines.next() {
        let line = line?;

        let part = Part::parse(&line);

        let mut name = "in".to_string();
        while name != "R" && name != "A" {
            let wf = workflows.get(&name).unwrap();
            for rule in &wf.rules {
                if rule.check(&part) {
                    name = rule.target.to_string();
                    break;
                }
            }
        }

        if name == "A" {
            total += part.value();
        }
    }

    println!("{total}");

    let mut live = Vec::new();
    live.push((
        "in".to_string(),
        RangePart{
            x: Range{ start: 1, end: 4001 },
            m: Range{ start: 1, end: 4001 },
            a: Range{ start: 1, end: 4001 },
            s: Range{ start: 1, end: 4001 },
        },
    ));

    let mut accept = Vec::new();

    while let Some((name, range)) = live.pop() {
        let wf = workflows.get(&name).unwrap();
        let mut range = range;
        for rule in &wf.rules {
            let (pass, fail) = rule.check_range(&range);

            if !pass.is_empty() {
                match rule.target.as_str() {
                    "A" => { accept.push(pass); },
                    "R" => { },
                    _ =>  { live.push((rule.target.to_string(), pass)); },
                }
            }

            range = fail;
        }
        assert!(range.is_empty());
    }

    let mut total = 0;
    for a in &accept {
        total += a.count();
    }

    println!("{total}");

    Ok(())
}
