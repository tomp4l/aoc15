use std::str::FromStr;

use super::day::*;

pub struct Instance;

#[derive(Debug)]
struct Present {
    x: u32,
    y: u32,
    z: u32,
}

impl FromStr for Present {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s
            .split('x')
            .map(|s| s.parse::<u32>().map_err(|e| e.to_string()))
            .collect::<Result<Vec<_>, String>>()?;

        if split.len() == 3 {
            Ok(Present {
                x: split[0],
                y: split[1],
                z: split[2],
            })
        } else {
            Err(format!("Wrong format for present: {}", s))
        }
    }
}

impl Present {
    fn paper_needed(&self) -> u32 {
        let surface = 2 * (self.x * self.y + self.x * self.z + self.y * self.z);
        let slack = (self.x * self.y).min(self.x * self.z).min(self.y * self.z);
        surface + slack
    }

    fn ribbon_needed(&self) -> u32 {
        let perimeter = 2 * (self.x + self.y).min(self.x + self.z).min(self.y + self.z);
        let volume = self.x * self.y * self.z;

        perimeter + volume
    }
}

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let parsed = lines
            .iter()
            .map(|l| Present::from_str(l))
            .collect::<Result<Vec<_>, _>>()?;

        let part1: u32 = parsed.iter().map(|p| p.paper_needed()).sum();
        let part2: u32 = parsed.iter().map(|p| p.ribbon_needed()).sum();

        Ok(DayResult {
            part1: part1.to_string(),
            part2: Some(part2.to_string()),
        })
    }
}
