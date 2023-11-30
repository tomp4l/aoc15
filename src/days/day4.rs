use super::day::*;

pub struct Instance;

fn hash(secret: &str, zeros: usize) -> u32 {
    let mut i = 0;

    while format!("{:x}", md5::compute(secret.to_owned() + &i.to_string()))
        .chars()
        .take_while(|c| *c == '0')
        .count()
        < zeros
    {
        i += 1;
    }
    i
}

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let secret = &lines[0];
        Ok(DayResult {
            part1: hash(secret, 5).to_string(),
            part2: Some(hash(secret, 6).to_string()),
        })
    }
}
