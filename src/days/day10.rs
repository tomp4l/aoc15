use super::day::*;

pub struct Instance;

fn step(s: &str) -> String {
    if s.is_empty() {
        return "".to_owned();
    }
    let mut chars = s.chars();
    let mut current = chars.next().unwrap();
    let mut count = 1;
    let mut ret = "".to_owned();
    for c in chars {
        if c == current {
            count += 1;
        } else {
            ret += &(count.to_string() + &current.to_string());
            current = c;
            count = 1;
        }
    }
    ret += &(count.to_string() + &current.to_string());
    ret
}

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let start = &lines[0];

        let mut current = start.to_owned();

        for _ in 0..40 {
            current = step(&current);
        }
        let part1 = current.len().to_string();
        for _ in 0..10 {
            current = step(&current);
        }
        let part2 = current.len().to_string();

        Ok(DayResult {
            part1,
            part2: Some(part2),
        })
    }
}
