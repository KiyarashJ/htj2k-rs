pub fn bitplane_specify(coefficient: u32) -> u32 {
        32 - coefficient.leading_zeros()
}
