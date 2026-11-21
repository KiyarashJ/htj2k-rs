use crate::ht_cleanup::{
    prepared_cleanup_struct::PreparedCleanup, quad::{QuadError}, subband_preparation::prepare_subband, types_subbands::{FourSubbands},
};



pub fn handling_fns(
    code_blocks: &FourSubbands,
) -> Result<PreparedCleanup, QuadError> {

    Ok(PreparedCleanup {
        ll: prepare_subband(&code_blocks.0)?,
        lh: prepare_subband(&code_blocks.1)?,
        hl: prepare_subband(&code_blocks.2)?,
        hh: prepare_subband(&code_blocks.3)?,
    })
}