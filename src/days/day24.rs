use itertools::Itertools;

use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let numbers = lines
            .iter()
            .map(|l| l.parse::<u16>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;

        let sum: u16 = numbers.iter().sum();
        let target_a = sum / 3;
        let target_b = sum / 4;

        let mut part1 = 0;
        let mut part2 = 0;
        for i in 1.. {
            if let Some(n) = numbers
                .iter()
                .permutations(i)
                .filter(|x| x.iter().cloned().sum::<u16>() == target_a)
                .map(|i| i.iter().map(|&&i| u64::from(i)).product::<u64>())
                .min()
            {
                part1 = n;
                break;
            }
        }

        for i in 1.. {
            if let Some(n) = numbers
                .iter()
                .permutations(i)
                .filter(|x| x.iter().cloned().sum::<u16>() == target_b)
                .map(|i| i.iter().map(|&&i| u64::from(i)).product::<u64>())
                .min()
            {
                part2 = n;
                break;
            }
        }

        Ok(DayResult {
            part1: part1.to_string(),
            part2: Some(part2.to_string()),
        })
    }
}
