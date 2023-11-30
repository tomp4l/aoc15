use std::str::FromStr;

use super::day::*;

pub struct Instance;

#[derive(Debug)]
struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl FromStr for Ingredient {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(": ");
        let _ = split.next();
        let mut split = split.next().ok_or("missing ingredients")?.split(", ");
        let capacity = split
            .next()
            .ok_or("missing capacity")?
            .split(' ')
            .nth(1)
            .ok_or("missing capacity value")?
            .parse::<i32>()
            .map_err(|e| e.to_string())?;
        let durability = split
            .next()
            .ok_or("missing durability")?
            .split(' ')
            .nth(1)
            .ok_or("missing durability value")?
            .parse::<i32>()
            .map_err(|e| e.to_string())?;
        let flavor = split
            .next()
            .ok_or("missing flavor")?
            .split(' ')
            .nth(1)
            .ok_or("missing flavor value")?
            .parse::<i32>()
            .map_err(|e| e.to_string())?;
        let texture = split
            .next()
            .ok_or("missing texture")?
            .split(' ')
            .nth(1)
            .ok_or("missing texture value")?
            .parse::<i32>()
            .map_err(|e| e.to_string())?;
        let calories = split
            .next()
            .ok_or("missing calories")?
            .split(' ')
            .nth(1)
            .ok_or("missing calories value")?
            .parse::<i32>()
            .map_err(|e| e.to_string())?;

        Ok(Ingredient {
            capacity,
            durability,
            flavor,
            texture,
            calories,
        })
    }
}

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let ingredients = lines
            .iter()
            .map(|l| l.parse::<Ingredient>())
            .collect::<Result<Vec<_>, _>>()?;

        let mut combinations = vec![Vec::new()];
        for _ in 0..(ingredients.len() - 1) {
            combinations = combinations
                .iter()
                .flat_map(|l| {
                    (0..=(100 - l.iter().sum::<i32>()))
                        .map(|i| {
                            let mut c = l.clone();
                            c.push(i);
                            c
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        }

        combinations.iter_mut().for_each(|l| {
            l.push(100 - l.iter().sum::<i32>());
        });

        let mut max = 0;
        let mut max_restricted = 0;

        for counts in combinations.iter() {
            let recipe = counts
                .iter()
                .zip(&ingredients)
                .map(|(c, i)| {
                    (
                        c * i.capacity,
                        c * i.durability,
                        c * i.flavor,
                        c * i.texture,
                        c * i.calories,
                    )
                })
                .fold((0, 0, 0, 0, 0), |a, b| {
                    (a.0 + b.0, a.1 + b.1, a.2 + b.2, a.3 + b.3, a.4 + b.4)
                });
            let sum = recipe.0.max(0) * recipe.1.max(0) * recipe.2.max(0) * recipe.3.max(0);
            max = max.max(sum);
            if recipe.4 == 500 {
                max_restricted = max_restricted.max(sum);
            }
        }

        Ok(DayResult {
            part1: max.to_string(),
            part2: Some(max_restricted.to_string()),
        })
    }
}
