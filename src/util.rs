use std::io;
use regex::Regex;

pub fn mod_remainder(k: u32, base: u32) -> (u32, u32) {
    ( k % base, k / base )
}

pub fn div_rem(a: u32, b: u32) -> (u32, u32) {
    (a / b, a % b)
}

pub fn mod_subtract(lhs: u32, rhs: u32, base: u32) -> (u32, u32) {
    let diff = lhs as i32 - rhs as i32;
    (diff.rem_euclid(base as i32) as u32, if diff >= 0 { 0 } else {1})
}

pub fn is_valid_time(input: &str) -> bool {
    let re = Regex::new(r"^(2[0-3]|1[0-9]|0?[0-9])(:([0-5][0-9]))?$").unwrap();
    re.is_match(input)
}

pub fn input_time() -> Result<String, String> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Input error");

    input = input.trim().to_string();

    if !is_valid_time(&input) {
        Err("Invalid time input".to_string())
    } else {
        Ok(input)
    }
}
