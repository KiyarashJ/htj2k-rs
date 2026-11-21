use crate::ht_cleanup::prepared_subband_struct::PreparedSubband;

pub struct PreparedCleanup {
    pub ll: PreparedSubband,
    pub lh: PreparedSubband,
    pub hl: PreparedSubband,
    pub hh: PreparedSubband,
}