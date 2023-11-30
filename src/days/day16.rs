use std::collections::HashMap;

use super::day::*;

pub struct Instance;

fn mfcsam_output() -> HashMap<String, u8> {
    let text = "samoyeds: 2
pomeranians: 3
akitas: 0
vizslas: 0
goldfish: 5
trees: 3
cars: 2
perfumes: 1";

    let mut output = HashMap::new();
    text.split('\n').for_each(|line| {
        let mut split = line.split(": ");
        let name = split.next().unwrap();
        let amount = split.next().unwrap().parse::<u8>().unwrap();
        output.insert(name.to_owned(), amount);
    });
    output
}

#[derive(Debug)]
struct Sue<'a> {
    number: u16,
    qualities: HashMap<&'a str, u8>,
}

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let sues: Vec<_> = lines
            .iter()
            .map(|line| {
                let mut split = line.split(' ');
                let number_raw = split.nth(1).unwrap();
                let number = number_raw[..number_raw.len() - 1].parse::<u16>().unwrap();
                let mut qualities = HashMap::new();
                while let (Some(name), Some(amount)) = (
                    split.next(),
                    split.next().and_then(|r| {
                        if r.contains(',') {
                            &r[..r.len() - 1]
                        } else {
                            r
                        }
                        .parse::<u8>()
                        .ok()
                    }),
                ) {
                    qualities.insert(&name[..name.len() - 1], amount);
                }

                Sue { number, qualities }
            })
            .collect();

        let output = mfcsam_output();
        let present_sue = sues.iter().find(|s| {
            s.qualities
                .iter()
                .all(|(n, v)| output.get(n.to_owned()).map_or(false, |a| a == v))
        });

        let real_present_sue = sues.iter().find(|s| {
            s.qualities.iter().all(|(n, v)| {
                output.get(n.to_owned()).map_or(false, |a| match *n {
                    "pomeranians" | "goldfish" => a > v,
                    "cats" | "trees" => a < v,
                    _ => a == v,
                })
            })
        });

        Ok(DayResult {
            part1: present_sue.map_or(0, |s| s.number).to_string(),
            part2: Some(real_present_sue.map_or(0, |s| s.number).to_string()),
        })
    }
}
