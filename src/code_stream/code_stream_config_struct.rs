#[derive(Debug, Clone, Copy)]
pub struct Codestreamconfig {
    pub width: usize,
    pub height: usize,
    pub bit_depth: u8,
    pub signed: bool,
    pub decomposition_levels: u8,
    pub code_block_width: usize,
    pub code_block_height: usize,
}