use std::error::Error;
use std::fmt;
use crate::dc_shifting::shift_error_enum::ShiftError;



impl fmt::Display for ShiftError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShiftError::InvalidBitsStored(bits) => {
                write!(f, "Invalid BitsStored for DC Shift: {bits}")
            }
        }
    }
}

impl Error for ShiftError {}

pub fn shift(
    bits_stored: u16,
    is_signed: bool,
    pixels: Vec<i32>,
) -> Result<Vec<i32>, ShiftError> {
    if !(1..=16).contains(&bits_stored) {
        return Err(ShiftError::InvalidBitsStored(bits_stored));
    }
    if is_signed {
        return Ok(pixels);
    }
    let offset = 1_i32
        .checked_shl(u32::from(bits_stored - 1))
        .ok_or(ShiftError::InvalidBitsStored(bits_stored))?;
    let shifted_pixels = pixels
        .into_iter()
        .map(|value| value - offset)
        .collect();

    Ok(shifted_pixels)
}
