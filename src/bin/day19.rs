use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

/// (category, value, target)
#[derive(Debug, Clone)]
enum Rule {
    Greater(String, i64, String),
    Less(String, i64, String),
}

impl Rule {
    fn matches(&self, part: &Part) -> bool {
        match self {
            Rule::Greater(category, value, _) => {
                if category == "x" {
                    part.x > *value
                } else if category == "m" {
                    part.m > *value
                } else if category == "a" {
                    part.a > *value
                } else if category == "s" {
                    part.s > *value
                } else {
                    unreachable!();
                }
            }
            Rule::Less(category, value, _) => {
                if category == "x" {
                    part.x < *value
                } else if category == "m" {
                    part.m < *value
                } else if category == "a" {
                    part.a < *value
                } else if category == "s" {
                    part.s < *value
                } else {
                    unreachable!();
                }
            }
        }
    }

    fn target(&self) -> String {
        match self {
            Rule::Greater(_, _, target) => target.clone(),
            Rule::Less(_, _, target) => target.clone(),
        }
    }

    fn reverse(&self) -> Self {
        match self {
            Rule::Greater(category, value, target) => {
                Rule::Less(category.to_string(), *value + 1, target.to_string())
            }
            Rule::Less(category, value, target) => {
                Rule::Greater(category.to_string(), *value - 1, target.to_string())
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
    default: String,
}

impl Workflow {
    fn process(&self, part: &Part) -> String {
        for rule in &self.rules {
            if rule.matches(part) {
                return rule.target();
            }
        }
        return self.default.clone();
    }
}

fn parse_workflow(s: &str) -> Workflow {
    let idx = s.find('{').unwrap();

    let mut result = Workflow {
        name: s[..idx].to_string(),
        rules: vec![],
        default: String::new(),
    };

    for rule in s[idx + 1..s.len() - 1].split_terminator(',') {
        if let Some((condition, target)) = rule.split_once(':') {
            if condition.contains('>') {
                let (category, value) = condition.split_once('>').unwrap();
                result.rules.push(Rule::Greater(
                    category.to_string(),
                    value.parse().unwrap(),
                    target.to_string(),
                ));
            } else if condition.contains('<') {
                let (category, value) = condition.split_once('<').unwrap();
                result.rules.push(Rule::Less(
                    category.to_string(),
                    value.parse().unwrap(),
                    target.to_string(),
                ));
            } else {
                unreachable!();
            }
        } else {
            result.default = rule.to_string();
        }
    }

    result
}

fn parse_part(s: &str) -> Part {
    let mut part = Part {
        x: 0,
        m: 0,
        a: 0,
        s: 0,
    };

    for rating in s[1..s.len() - 1].split(',') {
        let (category, value) = rating.split_once('=').unwrap();
        if category == "x" {
            part.x = value.parse().unwrap();
        } else if category == "m" {
            part.m = value.parse().unwrap();
        } else if category == "a" {
            part.a = value.parse().unwrap();
        } else if category == "s" {
            part.s = value.parse().unwrap();
        } else {
            unreachable!();
        }
    }

    part
}

/// Range of a part that would be accepted. Inclusive.
#[derive(Debug, Clone, Copy)]
struct PartRange {
    x: (i64, i64),
    m: (i64, i64),
    a: (i64, i64),
    s: (i64, i64),
}

fn merge_interval(lhs: (i64, i64), rhs: (i64, i64)) -> Option<(i64, i64)> {
    if lhs.0 <= rhs.1 && rhs.0 <= lhs.1 {
        Some((i64::max(lhs.0, rhs.0), i64::min(lhs.1, rhs.1)))
    } else {
        None
    }
}

impl PartRange {
    fn new() -> Self {
        Self {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        }
    }

    /// Returns a new range that would be accepted by both ranges. Returns None
    /// if the resulting range is empty.
    fn merge(&self, other: &PartRange) -> Option<Self> {
        Some(PartRange {
            x: merge_interval(self.x, other.x)?,
            m: merge_interval(self.m, other.m)?,
            a: merge_interval(self.a, other.a)?,
            s: merge_interval(self.s, other.s)?,
        })
    }

    fn size(&self) -> usize {
        let x = (self.x.1 - self.x.0 + 1) as usize;
        let m = (self.m.1 - self.m.0 + 1) as usize;
        let a = (self.a.1 - self.a.0 + 1) as usize;
        let s = (self.s.1 - self.s.0 + 1) as usize;
        x * m * a * s
    }

    fn from_rule(rule: &Rule) -> Self {
        let mut range = Self::new();
        match rule {
            Rule::Greater(category, value, _) => {
                if category == "x" {
                    range.x.0 = i64::max(range.x.0, *value + 1);
                } else if category == "m" {
                    range.m.0 = i64::max(range.m.0, *value + 1);
                } else if category == "a" {
                    range.a.0 = i64::max(range.a.0, *value + 1);
                } else if category == "s" {
                    range.s.0 = i64::max(range.s.0, *value + 1);
                } else {
                    unreachable!();
                }
            }
            Rule::Less(category, value, _) => {
                if category == "x" {
                    range.x.1 = i64::min(range.x.1, *value - 1);
                } else if category == "m" {
                    range.m.1 = i64::min(range.m.1, *value - 1);
                } else if category == "a" {
                    range.a.1 = i64::min(range.a.1, *value - 1);
                } else if category == "s" {
                    range.s.1 = i64::min(range.s.1, *value - 1);
                } else {
                    unreachable!();
                }
            }
        }
        range
    }
}

fn solve_range(
    name: &str,
    workflows: &HashMap<String, Workflow>,
    cache: &mut HashMap<String, Vec<PartRange>>,
) -> Vec<PartRange> {
    if name == "A" {
        return vec![PartRange::new()];
    }
    if name == "R" {
        return vec![];
    }

    // Check cached results.
    if let Some(result) = cache.get(name) {
        return result.clone();
    }

    let mut result = vec![];
    let mut range = PartRange::new();
    for rule in &workflows[name].rules {
        if let Some(range_after_applying_rule) = range.merge(&PartRange::from_rule(rule)) {
            for subrange in solve_range(&rule.target(), workflows, cache) {
                if let Some(candidate) = range_after_applying_rule.merge(&subrange) {
                    result.push(candidate);
                }
            }
        }

        if let Some(range_after_denying_rule) = range.merge(&PartRange::from_rule(&rule.reverse()))
        {
            range = range_after_denying_rule;
        } else {
            break;
        }
    }
    for subrange in solve_range(&workflows[name].default, workflows, cache) {
        if let Some(range_with_default_target) = range.merge(&subrange) {
            result.push(range_with_default_target);
        }
    }

    // Update cached results.
    cache.insert(name.to_string(), result.clone());

    result
}

fn main() {
    let input = std::fs::read("data/day19").unwrap();
    let input = String::from_utf8(input).unwrap();

    let mut workflows: HashMap<String, Workflow> = HashMap::new();
    let mut parts: Vec<Part> = vec![];
    let (workflow_lines, part_lines) = input.split_once("\r\n\r\n").unwrap();
    for workflow_line in workflow_lines.split_terminator("\r\n") {
        let workflow = parse_workflow(workflow_line);
        workflows.insert(workflow.name.clone(), workflow);
    }
    for part_line in part_lines.split_terminator("\r\n") {
        parts.push(parse_part(part_line));
    }

    let mut sum = 0;
    for part in &parts {
        let mut cur = "in".to_string();
        while cur != "A" && cur != "R" {
            cur = workflows[&cur].process(part);
        }
        if cur == "A" {
            sum += part.x + part.m + part.a + part.s;
        }
    }
    println!("sum = {sum}");

    let mut cache = HashMap::new();
    let range = solve_range("in", &workflows, &mut cache);
    let size: usize = range.iter().map(|r| r.size()).sum();
    dbg!(size);
}
