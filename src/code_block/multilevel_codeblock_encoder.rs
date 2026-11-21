use ndic_codestream::packet::BandBlocks;
use crate::{
    code_block::encode_one_subband::encode_one_subband, code_stream::packet_builder::{
        EncodedPacket, build_packets,
    }, errors::invalid::invalid, subbands::multilevel_subbands_struct::MultiLevelSubbands,
};
use ndic_core::{Result};


pub fn encode_multilevel_to_packets(
    subbands: &MultiLevelSubbands,
    bit_depth: u8,
) -> Result<Vec<EncodedPacket>> {
    let levels = subbands.levels;
    if levels > 5 {
        return Err(invalid("Maximum supported DWT levels is 5"));
    }
    if subbands.details.len() != usize::from(levels) {
        return Err(invalid(
            "Number of detail levels does not match decomposition levels"
        ));
    }
    let mut resolutions: Vec<Vec<BandBlocks>> =
        Vec::with_capacity(usize::from(levels) + 1);
    let ll_band = encode_one_subband(
        &subbands.ll,
        bit_depth,
        levels,
        0,
        0,
    )?;
    resolutions.push(vec![ll_band]);
    for resolution in 1..=levels {
        let detail_index = usize::from(
            levels - resolution
        );
        let detail = &subbands.details[detail_index];
        if detail.level != levels - resolution + 1 {
            return Err(invalid(
                "Unexpected detail-level ordering"
            ));
        }
        let hl_band = encode_one_subband(
            &detail.hl,
            bit_depth,
            levels,
            resolution,
            1,
        )?;
        let lh_band = encode_one_subband(
            &detail.lh,
            bit_depth,
            levels,
            resolution,
            2,
        )?;
        let hh_band = encode_one_subband(
            &detail.hh,
            bit_depth,
            levels,
            resolution,
            3,
        )?;
        resolutions.push(vec![
            hl_band,
            lh_band,
            hh_band,
        ]);
    }
    build_packets(&resolutions)
}
