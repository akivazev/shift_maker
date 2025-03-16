use crate::time::{Duration, Time};

pub fn fixed_shift_list(names: &[&str], shift_len: Duration, start_time: Time) -> Vec<(String, Time)> {
    let mut shift_list: Vec<(String, Time)> = Vec::with_capacity(names.len());

    let mut shift_start = start_time;

    for &name in names {
        shift_list.push((String::from(name), shift_start));
        shift_start = shift_start + shift_len;
    }

    shift_list
}

pub fn fixed_shift_list_cyclical(names: &[&str], shift_len: Duration, start_time: Time,
                                 shift_count: u32) -> Vec<(String, Time)> {
    let mut shift_list: Vec<(String, Time)> = Vec::with_capacity(names.len());

    let name_count = names.len() as u32;

    let mut shift_start = start_time;


    for i in 0..shift_count {
        shift_list.push((String::from(names[(i % name_count) as usize]), shift_start));
        shift_start = shift_start + shift_len;
    }

    shift_list
}

pub fn var_shift_list(names: &[&str], start_time: Time, duration: Duration)
    -> (Vec<(String, Time)>, Duration) {

    let shift_time = duration / names.len() as u32;

    (fixed_shift_list(names, shift_time, start_time), shift_time)
}