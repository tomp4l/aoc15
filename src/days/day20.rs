use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let target = lines[0].parse::<i32>().unwrap();

        let mut part1 = "".to_owned();
        let mut part2 = "".to_owned();

        for n in 1.. {
            let mut factors = vec![1, n];
            for i in 2.. {
                if i * i > n {
                    break;
                }
                if n % i == 0 {
                    factors.push(i);
                    if i * i != n {
                        factors.push(n / i);
                    }
                }
            }

            if part1.is_empty() {
                let sum: i32 = factors.iter().sum();

                if sum * 10 >= target {
                    part1 = n.to_string();
                    if !part2.is_empty() {
                        break;
                    }
                }
            }

            if part2.is_empty() {
                let sum: i32 = factors.iter().filter(|&i| n / i <= 50).sum();

                if sum * 11 >= target {
                    part2 = n.to_string();
                    if !part1.is_empty() {
                        break;
                    }
                }
            }
        }

        Ok(DayResult {
            part1,
            part2: Some(part2),
        })
    }
}
