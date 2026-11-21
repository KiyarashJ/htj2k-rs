#[cfg(test)]
mod tests {
    

    use crate::{code_block::multilevel_codeblock_encoder::encode_multilevel_to_packets, subbands::multilevel::multilevel_subband};

    fn test_image(
        width: usize,
        height: usize,
    ) -> Vec<Vec<i32>> {
        (0..height)
            .map(|y| {
                (0..width)
                    .map(|x| {
                        ((x * 37 + y * 17) % 4096) as i32 - 2048
                    })
                    .collect()
            })
            .collect()
    }

    #[test]
    fn encodes_all_five_levels_into_packets() {
        let image = test_image(64, 64);

        let subbands = multilevel_subband(
            &image,
            5,
        ).unwrap();

        let packets = encode_multilevel_to_packets(
            &subbands,
            12,
        ).unwrap();

        assert_eq!(packets.len(), 6);

        for (index, packet) in packets.iter().enumerate() {
            assert_eq!(packet.resolution, index as u8);
            assert!(!packet.data.is_empty());
        }
    }

    #[test]
    fn rejects_wrong_detail_count() {
        let image = test_image(64, 64);

        let mut subbands = multilevel_subband(
            &image,
            5,
        ).unwrap();

        subbands.details.pop();

        assert!(
            encode_multilevel_to_packets(
                &subbands,
                12,
            ).is_err()
        );
    }

    #[test]
    fn encodes_one_level() {
        let image = test_image(64, 64);

        let subbands = multilevel_subband(
            &image,
            1,
        ).unwrap();

        let packets = encode_multilevel_to_packets(
            &subbands,
            12,
        ).unwrap();

        assert_eq!(packets.len(), 2);
        assert_eq!(packets[0].resolution, 0);
        assert_eq!(packets[1].resolution, 1);
    }
}
