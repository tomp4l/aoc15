use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let line = &lines[0];
        let instructions = line
            .chars()
            .map(|c| match c {
                '(' => 1,
                ')' => -1,
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();

        let floor: i32 = instructions.iter().sum();
        let mut current_floor: i32 = 0;
        let mut instruction: usize = 0;

        for i in instructions.iter() {
            current_floor += i;
            instruction += 1;
            if current_floor == -1 {
                break;
            }
        }

        Ok(DayResult {
            part1: floor.to_string(),
            part2: Some(instruction.to_string()),
        })
    }
}
