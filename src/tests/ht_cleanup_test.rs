
#[cfg(test)]
mod tests {
    use crate::ht_cleanup::{encode_codeblock::encode_codeblock, ht_cleanup_adapter::k_max_for_band};

    use ndic_htj2k::{
        decode_block,
        sign_magnitude_to_coeff,
        BlockPasses,
    };

    fn round_trip(
        block: &[Vec<i32>],
        bit_depth: u8,
        levels: u8,
        resolution: u8,
        band: u8,
    ) {
        let k_max = k_max_for_band(
            bit_depth,
            levels,
            resolution,
            band,
        )
        .expect("K_max calculation failed");

        let coded = encode_codeblock(block, k_max)
            .expect("Cleanup encoding failed")
            .expect("Non-zero test block expected");

        let width = block[0].len();
        let height = block.len();

        let mut decoded = vec![0u32; width * height];

        let passes = BlockPasses::cleanup_only(
            coded.data.len()
        );

        decode_block(
            &coded.data,
            &mut decoded,
            width,
            height,
            width,
            u32::from(coded.missing_msbs),
            &passes,
            false,
        )
        .expect("Cleanup decoding failed");

        let shift = 31 - u32::from(k_max);

        let reconstructed: Vec<i32> = decoded
            .iter()
            .map(|&value| {
                sign_magnitude_to_coeff(value, shift)
            })
            .collect();

        let original: Vec<i32> = block
            .iter()
            .flatten()
            .copied()
            .collect();

        assert_eq!(
            reconstructed,
            original,
            "HT Cleanup round-trip mismatch"
        );
    }

    #[test]
    fn cleanup_round_trip_ll() {
        let block = vec![
            vec![100, -50, 25, 0, 12, -7, 3, 1],
            vec![-30, 60, 0, 15, -9, 4, 2, -1],
            vec![8, -12, 32, -64, 16, -8, 4, -2],
            vec![1, 2, 3, 4, -1, -2, -3, -4],
            vec![0, 0, 0, 0, 1, -1, 2, -2],
            vec![5, -5, 10, -10, 20, -20, 40, -40],
            vec![7, 6, 5, 4, 3, 2, 1, 0],
            vec![-7, -6, -5, -4, -3, -2, -1, 0],
        ];

        round_trip(
            &block,
            12,
            5,
            0,
            0,
        );
    }

    #[test]
    fn cleanup_round_trip_detail_band() {
        let block = vec![
            vec![100, -50, 25, 0, 12, -7, 3, 1],
            vec![-30, 60, 0, 15, -9, 4, 2, -1],
            vec![8, -12, 32, -64, 16, -8, 4, -2],
            vec![1, 2, 3, 4, -1, -2, -3, -4],
        ];

        round_trip(
            &block,
            12,
            5,
            5,
            3,
        );
    }

    #[test]
    fn zero_block_is_not_encoded() {
        let block = vec![
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ];

        let result = encode_codeblock(&block, 12)
            .expect("zero block check failed");

        assert!(result.is_none());
    }

    #[test]
    fn rejects_non_rectangular_block() {
        let block = vec![
            vec![1, 2, 3],
            vec![4, 5],
        ];

        assert!(encode_codeblock(&block, 12).is_err());
    }

    #[test]
    fn rejects_invalid_k_max() {
        let block = vec![
            vec![1, 2, 3, 4],
        ];

        assert!(encode_codeblock(&block, 0).is_err());
        assert!(encode_codeblock(&block, 31).is_err());
    }
}
