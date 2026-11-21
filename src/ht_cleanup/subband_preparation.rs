use crate::ht_cleanup::{analyze_blocks::analyze_blocks, pre_codeblock_info::pre_info_set, prepared_subband_struct::PreparedSubband, quad::{QuadError, build_quads}, types_subbands::SubbandBlocks};

pub fn prepare_subband(
    subband: &SubbandBlocks,
) -> Result<PreparedSubband, QuadError> {

    let quads = build_quads(subband.as_slice())?;
    let block_info = pre_info_set(subband);
    let analyses = analyze_blocks(&quads);
    Ok(PreparedSubband {
        block_info,
        quads,
        analyses,
    })
}