use crate::wavelet::wavelet::wavelet;

pub fn columns(m: &[Vec<i32>]) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let h = m.len();
    if h == 0 { return (vec![], vec![]); }
    let w = m[0].len();
    let mut low  = vec![vec![0; w]; (h + 1) / 2];
    let mut high = vec![vec![0; w]; h / 2];
    for c in 0..w {
        let col: Vec<i32> = m.iter().map(|r| r[c]).collect();
        let (lo, hi) = wavelet(&col);
        for (r, v) in lo.iter().enumerate() { low[r][c]  = *v; }
        for (r, v) in hi.iter().enumerate() { high[r][c] = *v; }
    }
    (low, high)
}