use crate::ht_cleanup::{codeblock_struct::Codeblock, quad::CodeblockQuads, quad_analysis_struct::QuadAnalysis};
pub struct PreparedSubband {
    pub block_info: Vec<Codeblock>,
    pub quads: Vec<CodeblockQuads>,
    pub analyses: Vec<Vec<QuadAnalysis>>,
}