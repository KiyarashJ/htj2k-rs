#[derive(Debug, Clone)]
pub struct DetailSubbands {
    pub level: u8,
    pub hl: Vec<Vec<i32>>,
    pub lh: Vec<Vec<i32>>,
    pub hh: Vec<Vec<i32>>,
}