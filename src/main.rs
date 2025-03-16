use crate::time::Time;

mod time;

fn main() {
    let t1 = Time::new(0, 0, 0);
    let t2 = Time::new(13, 22, 0);
    let t3 = Time::new(6, 0, 0);
    let xxxviii_min = Time::new(0, 38, 0);
    let t4 = t1 + t2;
    let t5 = t2 + t3;
    let t6 = t5 + t3;
    let t7 = t6 + xxxviii_min;

    println!("t1: {}", t1);
    println!("t2: {}", t2);
    println!("t3: {}", t3);
    println!("t1 + t2: {}", t4);
    println!("t2 + t3: {}", t5);
    println!("t2 + 2*t3: {}", t6);
    println!("t2 + 2*t3 + 38 minutes: {}", t7);

}
