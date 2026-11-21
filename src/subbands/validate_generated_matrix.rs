pub fn validate_matrix(image: &[Vec<i32>]) -> Result<(), String> {
    if image.is_empty() {
        return Err("Input image cannot be empty".to_string());
    }
    let width = image[0].len();
    if width == 0 {
        return Err("Input image width cannot be zero".to_string());
    }
    for (index, row) in image.iter().enumerate() {
        if row.len() != width {
            return Err(format!(
                "Input image is not rectangular at row {index}"
            ));
        }
    }
    Ok(())
}