pub fn magnitude_exponent(magnitude: u64) -> u8 {
    if magnitude == 0 {
        return 0;
    }
    let value = 2 * magnitude - 1;
    (u64::BITS - value.leading_zeros()) as u8
}