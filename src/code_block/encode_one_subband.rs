use ndic_codestream::packet::BandBlocks;
use crate::{code_block::matrix_generator::generate, errors::invalid::invalid, ht_cleanup::encode_subband_to_band::encode_subband_to_band};
use ndic_core::{Result};

const CODEBLOCK_SIZE: usize = 64;


pub fn encode_one_subband(
    subband: &[Vec<i32>],
    bit_depth: u8,
    decomposition_levels: u8,
    resolution: u8,
    band: u8,
) -> Result<BandBlocks> {
    if subband.is_empty() {
        return Err(invalid("Subband cannot be empty"));
    }
    let height = subband.len();
    let width = subband[0].len();
    if width == 0 {
        return Err(invalid("Subband width cannot be zero"));
    }
    for (row_index, row) in subband.iter().enumerate() {
        if row.len() != width {
            return Err(invalid(format!(
                "Non-rectangular subband at row {row_index}"
            )));
        }
    }
    let blocks_x = width.div_ceil(CODEBLOCK_SIZE);
    let blocks_y = height.div_ceil(CODEBLOCK_SIZE);
    let code_blocks = generate(subband);
    let expected_blocks = blocks_x
        .checked_mul(blocks_y)
        .ok_or_else(|| invalid("Code-block grid overflow"))?;
    if code_blocks.len() != expected_blocks {
        return Err(invalid(format!(
            "Code-block count mismatch: expected {expected_blocks}, got {}",
            code_blocks.len()
        )));
    }
    encode_subband_to_band(
        &code_blocks,
        blocks_x,
        blocks_y,
        bit_depth,
        decomposition_levels,
        resolution,
        band,
    )
}