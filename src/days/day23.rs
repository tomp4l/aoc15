use std::{collections::HashMap, str::FromStr};

use super::day::*;

pub struct Instance;

#[derive(Clone, Debug)]
enum Instruction {
    Half(String),
    Triple(String),
    Increment(String),
    Jump(i8),
    JumpIfEven(String, i8),
    JumpIfOne(String, i8),
}

struct Computer {
    registers: HashMap<String, u64>,
    instructions: Vec<Instruction>,
}

impl Computer {
    fn new(instructions: Vec<Instruction>) -> Self {
        Computer {
            registers: HashMap::new(),
            instructions,
        }
    }

    fn run(&mut self) {
        let mut pointer = 0;
        let instructions = self.instructions.clone();
        while let Some(i) = instructions.get(pointer) {
            match i {
                Instruction::Half(r) => {
                    let p = self.get_register(r.to_owned());
                    self.registers.insert(r.to_owned(), p / 2);
                    pointer += 1;
                }
                Instruction::Triple(r) => {
                    let p = self.get_register(r.to_owned());
                    self.registers.insert(r.to_owned(), p * 3);
                    pointer += 1;
                }
                Instruction::Increment(r) => {
                    let p = self.get_register(r.to_owned());
                    self.registers.insert(r.to_owned(), p + 1);
                    pointer += 1;
                }
                Instruction::Jump(j) => {
                    let x = pointer as i64 + *j as i64;
                    pointer = x as usize;
                }
                Instruction::JumpIfEven(r, j) => {
                    let r = self.get_register(r.to_owned());
                    if r % 2 == 0 {
                        let x = pointer as i64 + *j as i64;
                        pointer = x as usize;
                    } else {
                        pointer += 1;
                    }
                }
                Instruction::JumpIfOne(r, j) => {
                    let r = self.get_register(r.to_owned());
                    if r == 1 {
                        let x = pointer as i64 + *j as i64;
                        pointer = x as usize;
                    } else {
                        pointer += 1;
                    }
                }
            }
        }
    }

    fn get_register(&mut self, s: String) -> u64 {
        *self.registers.entry(s).or_insert(0)
    }
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(' ');

        let i = split.next();
        let a = split.next().ok_or("missing first arg")?;

        match i {
            Some("jio" | "jie") => {
                let n = split
                    .next()
                    .ok_or("missing second arg")?
                    .parse::<i8>()
                    .map_err(|e| e.to_string())?;

                if matches!(i, Some("jio")) {
                    Ok(Instruction::JumpIfOne(a[..1].to_owned(), n))
                } else {
                    Ok(Instruction::JumpIfEven(a[..1].to_owned(), n))
                }
            }
            Some("inc") => Ok(Instruction::Increment(a.to_owned())),
            Some("tpl") => Ok(Instruction::Triple(a.to_owned())),
            Some("hlf") => Ok(Instruction::Half(a.to_owned())),
            Some("jmp") => {
                let n = a.parse::<i8>().map_err(|e| e.to_string())?;
                Ok(Instruction::Jump(n))
            }
            _ => Err(format!("unexpected input: {}", s)),
        }
    }
}

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let instructions = lines
            .iter()
            .map(|l| l.parse::<Instruction>())
            .collect::<Result<Vec<_>, _>>()?;

        let mut computer = Computer::new(instructions);
        computer.run();

        let part1 = computer.registers["b"].to_string();

        computer.registers = HashMap::new();
        computer.registers.insert("a".to_owned(), 1);
        computer.run();

        Ok(DayResult {
            part1,
            part2: Some(computer.registers["b"].to_string()),
        })
    }
}
