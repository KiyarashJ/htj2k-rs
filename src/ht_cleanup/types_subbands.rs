
pub type CodeblockMatrix = Vec<Vec<i32>>;


pub type SubbandBlocks = Vec<CodeblockMatrix>;


pub type FourSubbands = (
    SubbandBlocks,
    SubbandBlocks,
    SubbandBlocks,
    SubbandBlocks,
);