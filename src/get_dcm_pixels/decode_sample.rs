pub fn decode_sample(
    raw: u32,
    bits_stored: u16,
    is_signed: bool,
) -> i32 {
    let mask = (1_u32 << bits_stored) - 1;
    let value = raw & mask;

    if !is_signed {
        return value as i32;
    }

    let sign_bit = 1_u32 << (bits_stored - 1);

    if value & sign_bit != 0 {
        (value as i32) - (1_i32 << bits_stored)
    } else {
        value as i32
    }
}