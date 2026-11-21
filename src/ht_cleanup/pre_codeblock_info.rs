use crate::ht_cleanup::{bitplane::bitplane_specify, codeblock_struct::Codeblock};

pub fn pre_info_set(subband: &Vec<Vec<Vec<i32>>>) -> Vec<Codeblock> {
    let mut code_blocks_calculated: Vec<Codeblock> = Vec::new();
    for block in subband {
        let height = block.len();

        let width = if height > 0 {
            block[0].len()
        } else {
            0
        };

        let mut large = 0u32;

        for row in block {
            for &value in row.iter() {
                if value.unsigned_abs() > large {
                    large = value.unsigned_abs();
                }
            }
        }
        let bitplanes = bitplane_specify(large);
        code_blocks_calculated.push(Codeblock {
            width,
            height,
            max_abs: large,
            bitplanes
        });

    }
    code_blocks_calculated
}