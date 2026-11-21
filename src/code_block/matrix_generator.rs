pub fn generate(subband: &[Vec<i32>]) -> Vec<Vec<Vec<i32>>> {
    let height = subband.len();
    if height == 0 {
        return Vec::new();
    }
    let width = subband[0].len();
    let mut code_blocks = Vec::new();
    for y in (0..height).step_by(64) {
        for x in (0..width).step_by(64) {
            let end_y = (y + 64).min(height);
            let end_x = (x + 64).min(width);
            let mut block = Vec::new();
            for row in &subband[y..end_y] {
                block.push(row[x..end_x].to_vec());
            }
            code_blocks.push(block);
        }
    }

    code_blocks
}