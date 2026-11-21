
#[cfg(test)]
mod tests {
    use crate::subbands::multilevel::multilevel_subband;

    fn make_image(
        width: usize,
        height: usize,
    ) -> Vec<Vec<i32>> {
        (0..height)
            .map(|y| {
                (0..width)
                    .map(|x| {
                        ((x * 13 + y * 7) % 1024) as i32 - 512
                    })
                    .collect()
            })
            .collect()
    }

    #[test]
    fn creates_five_decomposition_levels() {
        let image = make_image(64, 64);

        let result = multilevel_subband(&image, 5)
            .unwrap();

        assert_eq!(result.levels, 5);
        assert_eq!(result.details.len(), 5);

        assert_eq!(result.details[0].level, 1);
        assert_eq!(result.details[4].level, 5);

        assert_eq!(result.ll.len(), 2);
        assert_eq!(result.ll[0].len(), 2);
    }

    #[test]
    fn resolution_zero_contains_only_ll() {
        let image = make_image(64, 64);

        let result = multilevel_subband(&image, 5)
            .unwrap();

        let bands = result
            .bands_for_resolution(0)
            .unwrap();

        assert_eq!(bands.len(), 1);
    }

    #[test]
    fn resolution_one_contains_coarsest_details() {
        let image = make_image(64, 64);

        let result = multilevel_subband(&image, 5)
            .unwrap();

        let bands = result
            .bands_for_resolution(1)
            .unwrap();

        assert_eq!(bands.len(), 3);

        assert_eq!(bands[0], result.details[4].hl.as_slice());
        assert_eq!(bands[1], result.details[4].lh.as_slice());
        assert_eq!(bands[2], result.details[4].hh.as_slice());
    }

    #[test]
    fn resolution_five_contains_finest_details() {
        let image = make_image(64, 64);

        let result = multilevel_subband(&image, 5)
            .unwrap();

        let bands = result
            .bands_for_resolution(5)
            .unwrap();

        assert_eq!(bands[0], result.details[0].hl.as_slice());
        assert_eq!(bands[1], result.details[0].lh.as_slice());
        assert_eq!(bands[2], result.details[0].hh.as_slice());
    }

    #[test]
    fn rejects_too_many_levels() {
        let image = make_image(64, 64);

        assert!(
            multilevel_subband(&image, 6).is_err()
        );
    }

    #[test]
    fn rejects_small_image() {
        let image = make_image(16, 16);

        assert!(
            multilevel_subband(&image, 5).is_err()
        );
    }

    #[test]
    fn rejects_non_rectangular_image() {
        let image = vec![
            vec![1, 2, 3],
            vec![4, 5],
        ];

        assert!(
            multilevel_subband(&image, 1).is_err()
        );
    }
}
