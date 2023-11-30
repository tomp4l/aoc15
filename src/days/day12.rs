use serde_json::Value;

use super::day::*;

pub struct Instance;

fn sum_all(json: &Value) -> i64 {
    match json {
        Value::Null => 0,
        Value::Bool(_) => 0,
        Value::Number(n) => n.as_i64().unwrap(),
        Value::String(_) => 0,
        Value::Array(v) => v.iter().map(sum_all).sum(),
        Value::Object(v) => v.values().map(sum_all).sum(),
    }
}

fn sum_all_no_red(json: &Value) -> i64 {
    match json {
        Value::Null => 0,
        Value::Bool(_) => 0,
        Value::Number(n) => n.as_i64().unwrap(),
        Value::String(_) => 0,
        Value::Array(v) => v.iter().map(sum_all_no_red).sum(),
        Value::Object(v) => {
            if v.values().any(|v| v.as_str().is_some_and(|s| s == "red")) {
                0
            } else {
                v.values().map(sum_all_no_red).sum()
            }
        }
    }
}

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let json_string = &lines[0];
        let json: Value = serde_json::from_str(json_string).map_err(|e| e.to_string())?;

        let part1 = sum_all(&json).to_string();
        let part2 = sum_all_no_red(&json).to_string();

        Ok(DayResult {
            part1,
            part2: Some(part2),
        })
    }
}
