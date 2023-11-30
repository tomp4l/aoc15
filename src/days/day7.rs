use std::{collections::HashMap, str::FromStr};

use super::day::*;

pub struct Instance;

#[derive(Clone, Debug)]
enum Instruction {
    Set(String, String),
    And(String, String, String),
    Or(String, String, String),
    Not(String, String),
    Lshift(String, u8, String),
    Rshift(String, u8, String),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains("AND") || s.contains("OR") {
            let mut split = s.split(' ');
            let l = split.next().ok_or("missing left")?.to_owned();
            let op = split.next().ok_or("missing op")?;
            let r = split.next().ok_or("missing right")?.to_owned();
            split
                .next()
                .filter(|&i| i == "->")
                .ok_or("expected arrow")?;
            let t = split.next().ok_or("missing right")?.to_owned();

            if op == "AND" {
                Ok(Instruction::And(l, r, t))
            } else {
                Ok(Instruction::Or(l, r, t))
            }
        } else if s.contains("RSHIFT") || s.contains("LSHIFT") {
            let mut split = s.split(' ');
            let r = split.next().ok_or("missing left")?.to_owned();
            let op = split.next().ok_or("missing op")?;
            let s = split
                .next()
                .ok_or("missing right")?
                .parse::<u8>()
                .map_err(|e| e.to_string())?;
            split
                .next()
                .filter(|&i| i == "->")
                .ok_or("expected arrow")?;
            let t = split.next().ok_or("missing right")?.to_owned();

            if op == "RSHIFT" {
                Ok(Instruction::Rshift(r, s, t))
            } else {
                Ok(Instruction::Lshift(r, s, t))
            }
        } else if s.starts_with("NOT") {
            let mut split = s.split(' ');
            split.next();
            let f = split.next().ok_or("missing from")?.to_owned();
            split
                .next()
                .filter(|&i| i == "->")
                .ok_or("expected arrow")?;
            let t = split.next().ok_or("missing to")?.to_owned();
            Ok(Instruction::Not(f, t))
        } else {
            let mut split = s.split(' ');
            let f = split.next().ok_or("missing from")?.to_owned();
            split
                .next()
                .filter(|&i| i == "->")
                .ok_or("expected arrow")?;
            let t = split.next().ok_or("missing to")?.to_owned();

            Ok(Instruction::Set(f, t))
        }
    }
}

struct Wires(HashMap<String, u16>);

impl Wires {
    fn new() -> Self {
        let mut map = HashMap::new();
        for i in 0..u16::MAX {
            map.insert(i.to_string(), i);
        }
        Wires(map)
    }

    fn apply(&mut self, instructions: &[Instruction]) {
        let mut remaining = instructions.to_owned();

        while !remaining.is_empty() {
            remaining.retain(|i| match i {
                Instruction::Set(f, t) | Instruction::Not(f, t) => {
                    if let Some(v) = self.0.get(f) {
                        let v = if matches!(i, Instruction::Set(_, _)) {
                            *v
                        } else {
                            !*v
                        };
                        self.0.insert(t.to_owned(), v);
                        false
                    } else {
                        true
                    }
                }
                Instruction::And(l, r, t) | Instruction::Or(l, r, t) => {
                    if let (Some(l), Some(r)) = (self.0.get(l), self.0.get(r)) {
                        let v = if matches!(i, Instruction::And(_, _, _)) {
                            l & r
                        } else {
                            l | r
                        };
                        self.0.insert(t.to_owned(), v);
                        false
                    } else {
                        true
                    }
                }
                Instruction::Lshift(f, s, t) | Instruction::Rshift(f, s, t) => {
                    if let Some(v) = self.0.get(f) {
                        let v = if matches!(i, Instruction::Lshift(_, _, _)) {
                            *v << s
                        } else {
                            *v >> s
                        };
                        self.0.insert(t.to_owned(), v);
                        false
                    } else {
                        true
                    }
                }
            });
        }
    }

    fn get(&self, register: &str) -> Option<u16> {
        self.0.get(register).copied()
    }
}

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let mut instructions = lines
            .iter()
            .map(|l| l.parse::<Instruction>())
            .collect::<Result<Vec<_>, _>>()?;

        let mut r1 = Wires::new();
        r1.apply(&instructions);
        let part1 = r1.get("a").unwrap();

        instructions.push(Instruction::Set(part1.to_string(), "b".to_owned()));
        let mut r2 = Wires::new();
        r2.apply(&instructions);
        let part2 = r2.get("a").unwrap();

        Ok(DayResult {
            part1: part1.to_string(),
            part2: Some(part2.to_string()),
        })
    }
}
