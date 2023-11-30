use std::collections::HashSet;

use super::day::*;

pub struct Instance;

fn reverse(subject: &str, replacements: &Vec<(&str, &str)>) -> usize {
    let mut candidates = HashSet::new();
    let mut replacements = replacements.to_owned();
    replacements.sort_by_key(|v| v.1.len() - v.0.len());
    replacements.reverse();
    candidates.insert(subject.to_owned());
    let mut loops = 0;
    loop {
        if candidates.contains("e") {
            break;
        }

        let mut new_candidates = HashSet::<String>::new();
        for k in candidates.iter() {
            for (a, b) in &replacements {
                let new_replacements = replace_strings(k, b, a);
                let has_replacements = !new_replacements.is_empty();
                new_candidates.extend(new_replacements);
                if has_replacements {
                    break;
                }
            }
        }
        candidates = new_candidates;
        loops += 1;
    }

    loops
}

fn replace_strings(subject: &str, from: &str, to: &str) -> HashSet<String> {
    let mut search = subject;
    let mut pos = 0;
    let mut possibilities = HashSet::new();
    while let Some(i) = search.find(from) {
        pos += i;
        let replaced = format!("{}{}{}", &subject[..pos], to, &search[i + from.len()..]);
        possibilities.insert(replaced);

        search = &search[i + from.len()..];
        pos += from.len();
    }
    possibilities
}

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let mut replacements = Vec::new();

        let lines_iter = lines.iter();

        for line in lines_iter.as_ref() {
            let mut split = line.split(" => ");
            if let (Some(a), Some(b)) = (split.next(), split.next()) {
                replacements.push((a, b));
            } else {
                break;
            }
        }

        let subject = lines_iter.as_ref().last().unwrap();
        let mut possibilities = HashSet::new();

        for (a, b) in &replacements {
            possibilities.extend(replace_strings(subject, a, b));
        }

        Ok(DayResult {
            part1: possibilities.len().to_string(),
            part2: Some(reverse(subject, &replacements).to_string()),
        })
    }
}
