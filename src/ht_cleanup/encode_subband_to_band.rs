use ndic_codestream::packet::BandBlocks;
use ndic_core::{Result};
use crate::{errors::invalid::invalid, ht_cleanup::{encode_codeblock::encode_codeblock, ht_cleanup_adapter::k_max_for_band}};

pub fn encode_subband_to_band(
    blocks: &[Vec<Vec<i32>>],
    blocks_x: usize,
    blocks_y: usize,
    bit_depth: u8,
    decomposition_levels: u8,
    resolution: u8,
    band: u8,
) -> Result<BandBlocks> {
    let expected_count = blocks_x
        .checked_mul(blocks_y)
        .ok_or_else(|| invalid("Code-block grid size overflow"))?;
    if blocks.len() != expected_count {
        return Err(invalid(format!(
            "Code-block count mismatch: expected {expected_count}, got {}",
            blocks.len()
        )));
    }
    let k_max = k_max_for_band(
        bit_depth,
        decomposition_levels,
        resolution,
        band,
    )?;
    let mut encoded_blocks = Vec::with_capacity(blocks.len());
    for block in blocks {
        let encoded = encode_codeblock(block, k_max)?;
        encoded_blocks.push(encoded);
    }
    Ok(BandBlocks {
        nx: blocks_x,
        ny: blocks_y,
        blocks: encoded_blocks,
    })
}