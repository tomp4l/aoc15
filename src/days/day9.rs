use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let mut places = HashSet::<&str>::new();
        let mut distances = HashMap::<(&str, &str), usize>::new();
        for line in lines.iter() {
            let mut split = line.split(' ');
            let from = split.next().unwrap();
            let _ = split.next().unwrap();
            let to = split.next().unwrap();
            let _ = split.next().unwrap();
            let distance = split.next().unwrap().parse::<usize>().unwrap();

            places.insert(from);
            places.insert(to);

            distances.insert((from, to), distance);
            distances.insert((to, from), distance);
        }

        let min_max = places
            .iter()
            .permutations(places.len())
            .map(|p| {
                p.windows(2)
                    .map(|p| match p {
                        [&a, &b] => distances[&(a, b)],
                        _ => 0,
                    })
                    .sum::<usize>()
            })
            .minmax()
            .into_option();

        Ok(DayResult {
            part1: min_max.map(|t| t.0.to_string()).unwrap(),
            part2: Some(min_max.map(|t| t.1.to_string()).unwrap()),
        })
    }
}
