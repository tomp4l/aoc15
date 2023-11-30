use super::day::*;

pub struct Instance;

fn next(s: &str) -> String {
    let mut i = s.len() - 1;
    let mut ret = s.to_owned();
    loop {
        if &s[i..=i] == "z" {
            ret.replace_range(i..=i, "a");
            if i == 0 {
                break;
            }
            i -= 1;
        } else {
            let mut c = s.chars().nth(i).unwrap();
            c = (c as u8 + 1) as char;
            ret.replace_range(i..=i, &c.to_string());
            break;
        }
    }
    ret
}

fn is_good(s: &str) -> bool {
    if s.contains('i') || s.contains('o') || s.contains('l') {
        return false;
    }

    let mut chars = s.chars();
    let mut prev = chars.next().unwrap();
    let mut inc_count = 0;
    let mut inc = false;
    let (mut pairs, mut skip) = (0, false);
    for c in chars {
        if !skip && c == prev {
            skip = true;
            pairs += 1;
        } else {
            skip = false;
        }
        if prev as u8 == (c as u8 - 1) {
            inc_count += 1;
            if inc_count >= 2 {
                inc = true;
            }
        } else {
            inc_count = 0;
        }
        if inc && pairs >= 2 {
            return true;
        }
        prev = c;
    }

    false
}

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let start = &lines[0];
        let mut current = start.to_owned();
        let mut part1 = None;

        loop {
            if is_good(&current) {
                if part1.is_none() {
                    part1 = Some(current.to_owned())
                } else {
                    break;
                }
            }
            current = next(&current);
        }

        Ok(DayResult {
            part1: part1.unwrap(),
            part2: Some(current),
        })
    }
}
