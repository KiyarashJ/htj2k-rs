use crate::ht_cleanup::{analysis::analyze_quad, quad::CodeblockQuads, quad_analysis_struct::QuadAnalysis};

pub fn analyze_blocks(
    blocks: &[CodeblockQuads],
) -> Vec<Vec<QuadAnalysis>> {

    blocks
        .iter()
        .map(|block| {
            block.quads
                .iter()
                .map(analyze_quad)
                .collect()
        })
        .collect()
}