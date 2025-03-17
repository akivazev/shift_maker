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