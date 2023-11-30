use super::day::*;

pub struct Instance;

fn to_n(row: u32, column: u32) -> u32 {
    (1..(row + column - 1)).sum::<u32>() + column
}

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let coords = lines[0]
            .split(' ')
            .map(|l| l[..l.len().saturating_sub(1)].parse::<u32>())
            .filter_map(|i| i.ok())
            .collect::<Vec<_>>();

        let row = coords[0];
        let column = coords[1];

        let n = to_n(row, column);

        let mut hash: u64 = 20151125;
        for _ in 1..n {
            hash *= 252533;
            hash %= 33554393;
        }

        Ok(DayResult {
            part1: hash.to_string(),
            part2: None,
        })
    }
}
