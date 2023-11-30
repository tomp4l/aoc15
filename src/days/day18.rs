use std::str::FromStr;

use itertools::Itertools;

use super::day::*;

pub struct Instance;

#[derive(Clone)]
enum Light {
    On,
    Off,
}

#[derive(Clone)]
struct Lights {
    width: usize,
    height: usize,
    lights: Vec<Light>,
    stuck: bool,
}

impl FromStr for Lights {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.find('\n').ok_or("missing newlines")?;
        let height = s.lines().count();

        let mut lights = Vec::new();

        for c in s.chars() {
            match c {
                '.' => lights.push(Light::Off),
                '#' => lights.push(Light::On),
                _ => (),
            }
        }

        Ok(Lights {
            width,
            height,
            lights,
            stuck: false,
        })
    }
}

impl Lights {
    fn step(&mut self) {
        let mut lights = self.lights.clone();

        for i in 0..self.lights.len() {
            let l = &self.lights[i];
            let n = self.neighbours(i);
            let on_count = n.iter().filter(|l| matches!(l, Light::On)).count();
            match l {
                Light::On => {
                    if !(2..=3).contains(&on_count) {
                        lights[i] = Light::Off
                    }
                }
                Light::Off => {
                    if on_count == 3 {
                        lights[i] = Light::On
                    }
                }
            }
        }
        if self.stuck {
            lights[0] = Light::On;
            lights[self.width - 1] = Light::On;
            lights[(self.height - 1) * self.width] = Light::On;
            lights[self.height * self.width - 1] = Light::On;
        }
        self.lights = lights;
    }

    fn neighbours(&self, i: usize) -> Vec<Light> {
        let x = i % self.width;
        let y = i / self.width;

        let mut xs = vec![x];
        let mut ys = vec![y];

        if x > 0 {
            xs.push(x - 1);
        }
        if x + 1 < self.width {
            xs.push(x + 1);
        }
        if y > 0 {
            ys.push(y - 1);
        }
        if y + 1 < self.height {
            ys.push(y + 1);
        }

        let mut ret = Vec::new();
        for x1 in xs {
            for &y1 in ys.iter() {
                if x1 != x || y1 != y {
                    let i = y1 * self.width + x1;
                    ret.push(self.lights[i].clone());
                }
            }
        }
        ret
    }

    fn count(&self) -> usize {
        self.lights
            .iter()
            .filter(|l| matches!(l, Light::On))
            .count()
    }
}

impl ToString for Lights {
    fn to_string(&self) -> String {
        itertools::Itertools::intersperse(
            self.lights
                .iter()
                .map(|l| match l {
                    Light::On => '#',
                    Light::Off => '.',
                })
                .chunks(self.width)
                .into_iter()
                .map(|l| l.into_iter().collect()),
            "\n".to_owned(),
        )
        .collect()
    }
}

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let mut lights = lines
            .join("\n")
            .parse::<Lights>()
            .map_err(|e| e.to_string())?;

        let mut stuck_lights = lights.clone();
        stuck_lights.stuck = true;

        for _ in 0..100 {
            lights.step();
            stuck_lights.step();
        }

        let part1 = lights.count();
        let part2 = stuck_lights.count();

        Ok(DayResult {
            part1: part1.to_string(),
            part2: Some(part2.to_string()),
        })
    }
}
