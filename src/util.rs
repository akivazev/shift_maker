pub fn mod_remainder(k: u32, base: u32) -> (u32, u32) {
    ( k % base, k / base )
}

pub fn div_rem(a: u32, b: u32) -> (u32, u32) {
    (a / b, a % b)
}