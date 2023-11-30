use itertools::Itertools;

use super::day::*;

pub struct Instance;

fn good(s: &str) -> bool {
    let vowel_count = s
        .chars()
        .filter(|&c| c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u')
        .count();

    let contains_double = s.chars().zip(s.chars().skip(1)).any(|(a, b)| a == b);

    let banned = ["ab", "cd", "pq", "xy"];
    let no_banned = !banned.iter().any(|b| s.contains(b));

    vowel_count >= 3 && contains_double && no_banned
}

fn better(s: &str) -> bool {
    let contains_sandwich = s.chars().zip(s.chars().skip(2)).any(|(a, b)| a == b);

    let chars = s.chars().collect::<Vec<_>>();
    let pairs = chars.windows(2).map(String::from_iter).collect::<Vec<_>>();

    let mut found_pair = false;
    for p in pairs.iter().sorted().dedup() {
        let mut count = 0;
        let mut skip = false;
        for q in pairs.iter() {
            if !skip {
                if q == p {
                    count += 1;
                    skip = true;
                }
            } else {
                skip = false;
            }
        }
        if count > 1 {
            found_pair = true;
            break;
        }
    }

    contains_sandwich && found_pair
}

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let part1 = lines.iter().filter(|s| good(s.as_str())).count();
        let part2 = lines.iter().filter(|s| better(s.as_str())).count();

        Ok(DayResult {
            part1: part1.to_string(),
            part2: Some(part2.to_string()),
        })
    }
}
