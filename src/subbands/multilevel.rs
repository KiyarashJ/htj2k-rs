use crate::subbands::{detail_subbands_struct::DetailSubbands, multilevel_subbands_struct::MultiLevelSubbands, subband_creation::subband, validate_generated_matrix::validate_matrix};

pub fn multilevel_subband(
    image: &[Vec<i32>],
    levels: u8,
) -> Result<MultiLevelSubbands, String> {

    if levels > 5 {
        return Err(
            "Maximum supported decomposition level is 5".to_string()
        );
    }
    validate_matrix(image)?;
    let mut current_ll = image.to_vec();
    let mut details = Vec::with_capacity(
        usize::from(levels)
    );
    for level in 1..=levels {
        let height = current_ll.len();
        let width = current_ll[0].len();
        if height < 2 || width < 2 {
            return Err(format!(
                "Image is too small for DWT level {level}: {width}x{height}"
            ));
        }
        let result = subband(&current_ll);
        let ll = result.ll;
        let detail = DetailSubbands {
            level,
            hl: result.hl,
            lh: result.lh,
            hh: result.hh,
        };
        details.push(detail);
        current_ll = ll;
    }
    Ok(MultiLevelSubbands {
        ll: current_ll,
        details,
        levels,
    })
}

impl MultiLevelSubbands {
    pub fn bands_for_resolution(
        &self,
        resolution: u8,
    ) -> Option<Vec<&[Vec<i32>]>> {

        if resolution > self.levels {
            return None;
        }

        if resolution == 0 {
            return Some(vec![self.ll.as_slice()]);
        }

        let index = usize::from(
            self.levels - resolution
        );

        let detail = self.details.get(index)?;

        Some(vec![
            detail.hl.as_slice(),
            detail.lh.as_slice(),
            detail.hh.as_slice(),
        ])
    }
}
