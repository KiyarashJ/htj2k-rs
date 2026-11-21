
#[cfg(test)]
mod tests {
    use crate::get_dcm_pixels::decode_samples::decode_samples;


    #[test]
    fn unsigned_8bit_samples() {
        let raw = [0_u8, 127, 128, 255];

        let decoded = decode_samples(
            &raw,
            8,
            8,
            false,
        )
        .unwrap();

        assert_eq!(decoded, vec![0, 127, 128, 255]);
    }

    #[test]
    fn unsigned_12bit_samples_in_16bit_storage() {
        let raw_values = [0_u16, 2047, 2048, 4095];

        let raw: Vec<u8> = raw_values
            .iter()
            .flat_map(|v| v.to_ne_bytes())
            .collect();

        let decoded = decode_samples(
            &raw,
            16,
            12,
            false,
        )
        .unwrap();

        assert_eq!(decoded, vec![0, 2047, 2048, 4095]);
    }

    #[test]
    fn signed_12bit_samples_in_16bit_storage() {
        let raw_values = [0x0000_u16, 0x07FF, 0x0800, 0x0FFF];

        let raw: Vec<u8> = raw_values
            .iter()
            .flat_map(|v| v.to_ne_bytes())
            .collect();

        let decoded = decode_samples(
            &raw,
            16,
            12,
            true,
        )
        .unwrap();

        assert_eq!(decoded, vec![0, 2047, -2048, -1]);
    }

    #[test]
    fn rejects_invalid_bit_depth() {
        let raw = [0_u8, 1, 2];

        assert!(
            decode_samples(&raw, 8, 0, false).is_err()
        );

        assert!(
            decode_samples(&raw, 8, 9, false).is_err()
        );
    }
}
