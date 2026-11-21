use ndic_codestream::packet::CodedBlock;
use ndic_htj2k::{coeff_to_sign_magnitude, encode_block};
use ndic_core::{Result};
use crate::errors::invalid::invalid;


pub fn encode_codeblock(
    block: &[Vec<i32>],
    k_max: u8,
) -> Result<Option<CodedBlock>> {
    let height = block.len();
    if height == 0 {
        return Err(invalid("Code-block height cannot be zero"));
    }
    let width = block[0].len();
    if width == 0 {
        return Err(invalid("Code-block width cannot be zero"));
    }
    if width > 1024 || height > 1024 {
        return Err(invalid("Code-block dimensions exceed limits"));
    }
    let area = width
        .checked_mul(height)
        .ok_or_else(|| invalid("Code-block area overflow"))?;
    if area > 4096 {
        return Err(invalid("Code-block area must not exceed 4096"));
    }
    if !(1..=30).contains(&k_max) {
        return Err(invalid("K_max must be between 1 and 30"));
    }
    for (row_index, row) in block.iter().enumerate() {
        if row.len() != width {
            return Err(invalid(format!(
                "Non-rectangular code-block: row {row_index} has invalid width"
            )));
        }
    }
    let max_abs = block
        .iter()
        .flatten()
        .map(|&value| value.unsigned_abs())
        .max()
        .unwrap_or(0);
    if max_abs == 0 {
        return Ok(None);
    }
    let required_bits = u32::BITS - max_abs.leading_zeros();
    if required_bits > u32::from(k_max) {
        return Err(invalid(format!(
            "Coefficient magnitude needs {required_bits} bits, but K_max is {k_max}"
        )));
    }
    let shift = 31 - u32::from(k_max);
    let mut sign_magnitude = Vec::with_capacity(area);
    for row in block {
        for &coefficient in row {
            sign_magnitude.push(
                coeff_to_sign_magnitude(coefficient, shift)
            );
        }
    }
    let missing_msbs = u32::from(k_max) - 1;
    let data = encode_block(
        &sign_magnitude,
        width,
        height,
        width,
        missing_msbs,
    )?;
    let len_cleanup = u32::try_from(data.len())
        .map_err(|_| invalid("Cleanup segment is too large"))?;
    Ok(Some(CodedBlock {
        missing_msbs: (k_max - 1),
        num_passes: 1,
        len_cleanup,
        len_refinement: 0,
        data,
    }))
}
