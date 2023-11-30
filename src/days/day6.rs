use std::str::FromStr;

use super::day::*;

pub struct Instance;

enum Op {
    On,
    Off,
    Toggle,
}

struct Point(u16, u16);

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let x = parts
            .next()
            .ok_or("Missing x coord")?
            .parse::<u16>()
            .map_err(|i| i.to_string())?;
        let y = parts
            .next()
            .ok_or("Missing u coord")?
            .parse::<u16>()
            .map_err(|i| i.to_string())?;
        Ok(Point(x, y))
    }
}

struct Instruction {
    op: Op,
    from: Point,
    to: Point,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (op, maybe_rest) = if s.starts_with("turn on ") {
            (Op::On, s.strip_prefix("turn on "))
        } else if s.starts_with("turn off ") {
            (Op::Off, s.strip_prefix("turn off "))
        } else {
            (Op::Toggle, s.strip_prefix("toggle "))
        };
        let rest = maybe_rest.ok_or(format!("Bad prefix: {}", s))?;

        let mut split = rest.split(' ');
        let from = split
            .next()
            .ok_or("Missing first coord")?
            .parse::<Point>()?;
        let _ = split.next().ok_or("Missing second coord");
        let to = split
            .next()
            .ok_or("Missing second coord")?
            .parse::<Point>()?;

        Ok(Instruction { op, from, to })
    }
}

struct LightsOnOff(Vec<bool>);

impl LightsOnOff {
    fn new() -> Self {
        LightsOnOff(vec![false; 1_000_000])
    }

    fn apply(&mut self, instruction: &Instruction) {
        for x in instruction.from.0..=instruction.to.0 {
            for y in instruction.from.1..=instruction.to.1 {
                let cell = self
                    .0
                    .get_mut(usize::from(x) + 1000 * usize::from(y))
                    .unwrap();
                match instruction.op {
                    Op::On => *cell = true,
                    Op::Off => *cell = false,
                    Op::Toggle => *cell = !*cell,
                }
            }
        }
    }

    fn count(&self) -> usize {
        self.0.iter().filter(|&&i| i).count()
    }
}

struct LightsBright(Vec<u8>);

impl LightsBright {
    fn new() -> Self {
        LightsBright(vec![0; 1_000_000])
    }

    fn apply(&mut self, instruction: &Instruction) {
        for x in instruction.from.0..=instruction.to.0 {
            for y in instruction.from.1..=instruction.to.1 {
                let cell = self
                    .0
                    .get_mut(usize::from(x) + 1000 * usize::from(y))
                    .unwrap();
                match instruction.op {
                    Op::On => *cell += 1,
                    Op::Off => {
                        if *cell > 0 {
                            *cell -= 1
                        }
                    }
                    Op::Toggle => *cell += 2,
                }
            }
        }
    }

    fn total(&self) -> u32 {
        self.0.iter().map(|&i| u32::from(i)).sum()
    }
}

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let parsed = lines
            .iter()
            .map(|l| l.parse::<Instruction>())
            .collect::<Result<Vec<_>, _>>()?;

        let mut lights = LightsOnOff::new();
        parsed.iter().for_each(|i| lights.apply(i));

        let mut lights_bright = LightsBright::new();
        parsed.iter().for_each(|i| lights_bright.apply(i));

        Ok(DayResult {
            part1: lights.count().to_string(),
            part2: Some(lights_bright.total().to_string()),
        })
    }
}
