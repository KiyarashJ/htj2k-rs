use crate::{subbands::{cols::columns, subbands_struct::Subbands}, wavelet::wavelet::wavelet};

pub fn subband(img: &[Vec<i32>]) -> Subbands {
    let mut l_rows = vec![];
    let mut h_rows = vec![];
    for row in img {
        let (lo, hi) = wavelet(row);
        l_rows.push(lo);
        h_rows.push(hi);
    }
    let (ll, lh) = columns(&l_rows);
    let (hl, hh) = columns(&h_rows);
    Subbands { ll, hl, lh, hh }
}