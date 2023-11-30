use super::day::*;

pub struct Instance;

fn combinations(target: u8, pots: &Vec<u8>) -> Vec<Vec<u8>> {
    let mut pots = pots.to_owned();
    let mut cs = Vec::new();
    while let Some(p) = pots.pop() {
        match p.cmp(&target) {
            std::cmp::Ordering::Less => {
                let mut rest = combinations(target - p, &pots);
                rest.iter_mut().for_each(|v| v.push(p));
                cs.extend(rest);
            }
            std::cmp::Ordering::Equal => cs.push(vec![p]),
            std::cmp::Ordering::Greater => (),
        }
    }

    cs
}

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let pot_sizes = lines
            .iter()
            .map(|l| l.parse::<u8>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;

        let combinations = combinations(150, &pot_sizes);
        let min_combination = combinations.iter().map(|v| v.len()).min().unwrap();

        Ok(DayResult {
            part1: combinations.len().to_string(),
            part2: Some(
                combinations
                    .iter()
                    .filter(|v| v.len() == min_combination)
                    .count()
                    .to_string(),
            ),
        })
    }
}
