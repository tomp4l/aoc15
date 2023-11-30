use std::collections::HashMap;

use itertools::Itertools;

use super::day::*;

pub struct Instance;

fn max_happiness(unique_names: &Vec<&str>, happiness_changes: &HashMap<(&str, &str), i32>) -> i32 {
    let mut totals = Vec::new();
    for p in unique_names.iter().permutations(unique_names.len()) {
        let total = p
            .windows(2)
            .map(|i| match i {
                [a, b] => {
                    happiness_changes.get(&(a, b)).unwrap()
                        + happiness_changes.get(&(b, a)).unwrap()
                }
                _ => unreachable!(),
            })
            .sum::<i32>();
        let first = p.first().unwrap();
        let last = p.last().unwrap();

        totals.push(
            total
                + happiness_changes.get(&(first, last)).unwrap()
                + happiness_changes.get(&(last, first)).unwrap(),
        );
    }

    *totals.iter().max().unwrap()
}

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let mut happiness_changes = HashMap::<(&str, &str), i32>::new();

        for line in lines.iter() {
            let mut split = line.split(' ');
            let subject = split.next().unwrap();
            let gain_loss = if split.nth(1).unwrap() == "gain" {
                1
            } else {
                -1
            };
            let amount = split.next().unwrap().parse::<i32>().unwrap();
            let target = split.last().unwrap();
            let target_clean = &target[..target.len() - 1]; //strip .

            happiness_changes.insert((subject, target_clean), gain_loss * amount);
        }

        let mut unique_names: Vec<_> = happiness_changes
            .keys()
            .flat_map(|(a, b)| vec![*a, *b])
            .sorted()
            .unique()
            .collect();

        let part1 = max_happiness(&unique_names, &happiness_changes).to_string();

        let me = "me";
        for name in unique_names.iter() {
            happiness_changes.insert((me, name), 0);
            happiness_changes.insert((name, me), 0);
        }
        unique_names.push(me);

        let part2 = max_happiness(&unique_names, &happiness_changes).to_string();

        Ok(DayResult {
            part1,
            part2: Some(part2),
        })
    }
}
