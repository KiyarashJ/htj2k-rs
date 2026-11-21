
#[cfg(test)]
mod tests {
    use crate::output::validate_code_stream::validate_codestream;

    #[test]
    fn rejects_invalid_codestream() {
        let data = vec![1, 2, 3, 4, 5];

        assert!(validate_codestream(&data).is_err());
    }

    #[test]
    fn rejects_missing_eoc() {
        let mut data = vec![
            0xFF, 0x4F,
            0xFF, 0x51,
        ];

        data.resize(50, 0);

        assert!(validate_codestream(&data).is_err());
    }
}
