use std::collections::HashSet;

use super::day::*;

pub struct Instance;

enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn from_char(c: char) -> Result<Direction, String> {
        match c {
            '^' => Ok(Direction::North),
            '>' => Ok(Direction::East),
            '<' => Ok(Direction::West),
            'v' => Ok(Direction::South),
            c => Err(format!("Unknown char {}", c)),
        }
    }
}

#[derive(Eq, Hash, PartialEq, Clone)]
struct Position(i32, i32);

impl Position {
    fn step(&self, direction: &Direction) -> Position {
        let Position(x, y) = &self;
        match direction {
            Direction::North => Position(*x, y + 1),
            Direction::East => Position(x + 1, *y),
            Direction::South => Position(*x, y - 1),
            Direction::West => Position(x - 1, *y),
        }
    }
}

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let line = &lines[0];
        let parsed = line
            .chars()
            .map(Direction::from_char)
            .collect::<Result<Vec<_>, _>>()?;

        let mut visited = HashSet::<Position>::new();
        let mut current = Position(0, 0);
        visited.insert(current.clone());

        let mut robo_visited = HashSet::<Position>::new();
        let mut santa_current = Position(0, 0);
        let mut robo_current = Position(0, 0);
        let mut is_santa = true;
        visited.insert(santa_current.clone());

        for d in parsed {
            current = current.step(&d);
            visited.insert(current.clone());
            if is_santa {
                santa_current = santa_current.step(&d);
                robo_visited.insert(santa_current.clone());
            } else {
                robo_current = robo_current.step(&d);
                robo_visited.insert(robo_current.clone());
            }
            is_santa = !is_santa;
        }

        Ok(DayResult {
            part1: visited.len().to_string(),
            part2: Some(robo_visited.len().to_string()),
        })
    }
}
