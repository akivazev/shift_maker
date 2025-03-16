use crate::time::{Duration, Time};
use crate::shift_list::{fixed_shift_list, fixed_shift_list_cyclical, var_shift_list};

mod time;
mod shift_list;
mod util;

fn main() {
    let start_time = Time::new(19, 0, 0);
    let shift_time = Duration::new(1, 15, 0);
    let rotation_dur = Duration::new(10, 0, 0);
    let names = ["Collins", "Akiva", "Nadav", "Matan", "Yaron", "Yarom", "Babich",
        "Moshiko", "Caspi"];

    let (shift_list, shift_len) = var_shift_list(&names, start_time, rotation_dur);

    for (name, time) in shift_list {
        println!("{}-{} {}", time, time + shift_len, name);
    }

}
