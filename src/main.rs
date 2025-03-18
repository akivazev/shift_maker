use std::{env, io};
use crate::time::{Duration, Time};
use crate::shift_list::{fixed_shift_list, fixed_shift_list_cyclical, interval_fixed_shift_list, interval_var_shift_list, var_shift_list};
use crate::util::input_time;

mod time;
mod shift_list;
mod util;

fn print_fixed_shift_list() {
    let mut input = String::new();

    println!("Enter start time (HH or HH:MM):");

    let start_time_str = input_time().expect("Invalid start time");

    let start_time = Time::from_string(&start_time_str).expect("Invalid start time");

    println!("Enter end time (HH or HH:MM):");

    let end_time_str = input_time().expect("Invalid end time");

    let end_time = Time::from_string(&end_time_str).expect("Invalid end time");

    println!("Enter shift length (HH or HH:MM):");

    let shift_length_str = input_time().expect("Invalid shift length");

    let shift_length = Duration::from_string(&shift_length_str).expect("Invalid shift length");

    println!("Enter names separated by whitespace:");

    io::stdin().read_line(&mut input)
        .expect("Input error");

    let names: Vec<&str> = input.split_whitespace().collect();

    let shift_list = interval_fixed_shift_list(&names, shift_length, start_time, end_time);

    for (name, start, end) in shift_list {
        println!("{}-{} {}", start, end, name);
    }
}

fn print_shift_list() {
    let mut input = String::new();

    println!("Enter start time (HH or HH:MM):");

    let start_time_str = input_time().expect("Invalid start time");

    let start_time = Time::from_string(&start_time_str).expect("Invalid start time");

    println!("Enter end time (HH or HH:MM):");

    let end_time_str = input_time().expect("Invalid end time");

    let end_time = Time::from_string(&end_time_str).expect("Invalid end time");

    println!("Enter names separated by whitespace:");

    io::stdin().read_line(&mut input)
        .expect("Input error");

    let names: Vec<&str> = input.split_whitespace().collect();

    let shift_list = interval_var_shift_list(&names, start_time, end_time);

    for (name, start, end) in shift_list {
        println!("{}-{} {}", start, end, name);
    }
}

fn main() {
    // let start_time = Time::new(19, 0, 0);
    // let end_time = Time::new(5, 0 , 0);
    // let shift_time = Duration::new(1, 1, 0);
    // let rotation_dur = Duration::new(10, 0, 0);
    // let names = ["Collins", "Akiva", "Nadav", "Matan", "Yaron", "Yarom", "Babich",
    //     "Moshiko", "Caspi"];
    //
    // let (shift_list, shift_len) = interval_fixed_shift_list(&names, shift_time, start_time, start_time);
    //
    // for (name, time) in shift_list {
    //     println!("{}-{} {}", time, time + shift_len, name);
    // }

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "--fixed-shift" {
        print_fixed_shift_list();
    } else {
        print_shift_list();
    }
}
