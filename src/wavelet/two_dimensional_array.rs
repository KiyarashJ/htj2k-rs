pub fn to_2d(pixels: &[i32], rows: usize, columns: usize) -> Vec<Vec<i32>> {
    let mut two_d_array: Vec<Vec<i32>> = Vec::new();

    for y in 0..columns {
        let start = y * rows;
        let end = start + rows;
        let row = pixels[start..end].to_vec();
        two_d_array.push(row);
    }
    two_d_array
}