
#[cfg(test)]
mod tests {
    use crate::dc_shifting::shifter::shift;


    #[test]
    fn shifts_unsigned_8bit() {
        let pixels = vec![0, 127, 128, 255];

        let result = shift(8, false, pixels).unwrap();

        assert_eq!(result, vec![-128, -1, 0, 127]);
    }

    #[test]
    fn shifts_unsigned_12bit() {
        let pixels = vec![0, 2047, 2048, 4095];

        let result = shift(12, false, pixels).unwrap();

        assert_eq!(result, vec![-2048, -1, 0, 2047]);
    }

    #[test]
    fn does_not_shift_signed_pixels() {
        let pixels = vec![-2048, -1, 0, 2047];

        let result = shift(12, true, pixels.clone()).unwrap();

        assert_eq!(result, pixels);
    }

    #[test]
    fn rejects_invalid_bits_stored() {
        assert!(shift(0, false, vec![0]).is_err());
        assert!(shift(17, false, vec![0]).is_err());
    }
}
