use super::day::*;

pub struct Instance;

fn decode_len(s: &str) -> usize {
    let mut len = 0;
    let mut chars = s[1..s.len() - 1].chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            if let Some(n) = chars.next() {
                if n == 'x' {
                    chars.next();
                    chars.next();
                }
            }
        }
        len += 1;
    }
    len
}

fn encode_len(s: &str) -> usize {
    s.chars().filter(|c| matches!(c, '\\' | '"')).count() + s.len() + 2
}

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let raw_total: usize = lines.iter().map(|l| l.len()).sum();
        let decode_total: usize = lines.iter().map(|l| decode_len(l)).sum();
        let encode_total: usize = lines.iter().map(|l| encode_len(l)).sum();

        Ok(DayResult {
            part1: (raw_total - decode_total).to_string(),
            part2: Some((encode_total - raw_total).to_string()),
        })
    }
}
