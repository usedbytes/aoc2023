use std::env;
use std::error::Error;
use std::fs::File;
use std::collections::HashMap;
use std::io::{self, BufRead};
use std::str::FromStr;

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

    Ok(())
}
