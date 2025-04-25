use std::collections::HashSet;
use std::io;
use regex::Regex;
use rand::seq::SliceRandom;
use rand::rng;

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

pub fn partial_shuffle<T: Clone>(target_vec: &mut Vec<T>, shuffle_indices: HashSet<usize>) {
    let max_index = target_vec.len();

    // Filter invalid indices from set
    let filtered_indices: HashSet<usize> = shuffle_indices.iter()
        .copied()
        .filter(|&i| i < max_index)
        .collect();

    // Clone elements at indices in set to temporary vec
    let mut temp_vec: Vec<T> = filtered_indices.iter().map(|&i| target_vec[i].clone()).collect();

    // Shuffle elements
    temp_vec.shuffle(&mut rng());

    // Set element at each index to random element from temporary vec
    for i in filtered_indices {
        target_vec[i] = temp_vec.pop().unwrap();
    }
}
