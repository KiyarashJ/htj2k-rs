
#[derive(Debug, Clone, Copy)]
pub struct QuadAnalysis {
    pub magnitudes: [u64; 4],
    pub signs: [bool; 4],
    pub exponents: [u8; 4],
    pub rho: u8,
    pub gamma: u8,
    pub magsgn_values: [u64; 4],
}