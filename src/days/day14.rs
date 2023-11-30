use std::{collections::HashMap, str::FromStr};

use super::day::*;

pub struct Instance;

struct Reindeer {
    name: String,
    speed: u16,
    run_time: u16,
    rest_time: u16,
}

impl FromStr for Reindeer {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(' ');
        let name = split.next().ok_or("missing name")?.to_owned();
        let speed = split
            .nth(2)
            .ok_or("missing speed")?
            .parse::<u16>()
            .map_err(|e| e.to_string())?;
        let run_time = split
            .nth(2)
            .ok_or("missing run time")?
            .parse::<u16>()
            .map_err(|e| e.to_string())?;
        let rest_time = split
            .nth(6)
            .ok_or("missing rest time")?
            .parse::<u16>()
            .map_err(|e| e.to_string())?;
        Ok(Reindeer {
            name,
            speed,
            run_time,
            rest_time,
        })
    }
}

impl Reindeer {
    fn distance(&self, seconds: u16) -> u16 {
        let mut running = RunningReindeer::new(self);
        for _ in 0..seconds {
            running.step();
        }
        running.distance
    }
}

enum ReindeerState {
    Running(u16),
    Resting(u16),
}

struct RunningReindeer<'a> {
    reindeer: &'a Reindeer,
    distance: u16,
    state: ReindeerState,
}

impl RunningReindeer<'_> {
    fn new(reindeer: &Reindeer) -> RunningReindeer<'_> {
        RunningReindeer {
            reindeer,
            distance: 0,
            state: ReindeerState::Running(reindeer.run_time),
        }
    }

    fn step(&mut self) {
        match self.state {
            ReindeerState::Running(left) => {
                self.distance += self.reindeer.speed;
                if left > 1 {
                    self.state = ReindeerState::Running(left - 1);
                } else {
                    self.state = ReindeerState::Resting(self.reindeer.rest_time);
                }
            }
            ReindeerState::Resting(left) => {
                if left > 1 {
                    self.state = ReindeerState::Resting(left - 1);
                } else {
                    self.state = ReindeerState::Running(self.reindeer.run_time);
                }
            }
        }
    }
}

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        const TIME_LIMIT: u16 = 2503;

        let reindeers = lines
            .iter()
            .map(|l| l.parse::<Reindeer>())
            .collect::<Result<Vec<_>, _>>()?;

        let max_distance = reindeers.iter().map(|r| r.distance(2503)).max().unwrap();

        let mut winners: HashMap<&str, u16> = HashMap::new();
        let mut running_reindeers: Vec<_> = reindeers.iter().map(RunningReindeer::new).collect();

        for _ in 0..TIME_LIMIT {
            running_reindeers.iter_mut().for_each(|r| r.step());
            let winner = running_reindeers.iter().max_by_key(|r| r.distance).unwrap();
            running_reindeers
                .iter()
                .filter(|r| r.distance == winner.distance)
                .for_each(|winner| {
                    winners
                        .entry(&winner.reindeer.name)
                        .and_modify(|i| *i += 1)
                        .or_insert(1);
                });
        }
        let max_points = winners.values().max().unwrap();

        Ok(DayResult {
            part1: max_distance.to_string(),
            part2: Some(max_points.to_string()),
        })
    }
}
