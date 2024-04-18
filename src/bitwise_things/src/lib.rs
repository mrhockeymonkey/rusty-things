

#[inline]
pub fn is_odd_modulo(input: u32) -> bool {
    input % 2 == 1
}

#[inline]
pub fn is_odd_bitwise_and(input: u32) -> bool {
    // 1101 and
    // 0001
    input & 1 == 1
}

#[inline]
pub fn is_pow_2(mut x: u64) -> bool {
    if x == 0 {
        true
    }
    else {
        while x % 2 == 0 {
            x /= 2;
        }
        x == 1
    }
}

#[inline]
pub fn is_pow_2_bitwise(x: u64) -> bool {
    x > 0 && (x & (x-1) == 0)
    // x - 1 will always result in 1s after lsb

    // 8 is   1000
    // 7 is   0111
    // and is 0000 so it is a power of 2
}