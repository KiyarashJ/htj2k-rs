use crate::subbands::detail_subbands_struct::DetailSubbands;

#[derive(Debug, Clone)]
pub struct MultiLevelSubbands {
    pub ll: Vec<Vec<i32>>,
    pub details: Vec<DetailSubbands>,
    pub levels: u8,
}