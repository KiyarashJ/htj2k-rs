use std::error::Error;
use std::fmt;

pub type Quad = [i32; 4];

#[derive(Debug, Clone)]
pub struct CodeblockQuads {
    pub sample_width: usize,
    pub sample_height: usize,

    pub quad_width: usize,
    pub quad_height: usize,

    pub quads: Vec<Quad>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuadError {
    NonRectangular {
        block_index: usize,
        row_index: usize,
        expected: usize,
        actual: usize,
    },
}

impl fmt::Display for QuadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QuadError::NonRectangular {
                block_index,
                row_index,
                expected,
                actual,
            } => write!(
                f,
                "Codeblock {} row {} has width {}, expected {}",
                block_index,
                row_index,
                actual,
                expected
            ),
        }
    }
}

impl Error for QuadError {}

pub fn build_quads(
    codeblocks: &[Vec<Vec<i32>>],
) -> Result<Vec<CodeblockQuads>, QuadError> {

    let mut all_blocks = Vec::with_capacity(codeblocks.len());

    for (block_index, block) in codeblocks.iter().enumerate() {

        let height = block.len();

        let width = block.first().map_or(0, |row| row.len());

        // Validate rectangular shape
        for (row_index, row) in block.iter().enumerate() {
            if row.len() != width {
                return Err(QuadError::NonRectangular {
                    block_index,
                    row_index,
                    expected: width,
                    actual: row.len(),
                });
            }
        }

        // HTJ2K dimensions are measured in quads here
        let quad_width = (width + 1) / 2;
        let quad_height = (height + 1) / 2;

        let mut quads =
            Vec::with_capacity(quad_width * quad_height);

        // Quad scan: left-to-right, top-to-bottom
        for qy in 0..quad_height {

            for qx in 0..quad_width {

                let y = qy * 2;
                let x = qx * 2;

                // Standard order:
                // 0 = top-left
                // 1 = bottom-left
                // 2 = top-right
                // 3 = bottom-right

                let top_left = block[y][x];

                let bottom_left = block
                    .get(y + 1)
                    .and_then(|row| row.get(x))
                    .copied()
                    .unwrap_or(0);

                let top_right = block[y]
                    .get(x + 1)
                    .copied()
                    .unwrap_or(0);

                let bottom_right = block
                    .get(y + 1)
                    .and_then(|row| row.get(x + 1))
                    .copied()
                    .unwrap_or(0);

                quads.push([
                    top_left,
                    bottom_left,
                    top_right,
                    bottom_right,
                ]);
            }
        }

        all_blocks.push(CodeblockQuads {
            sample_width: width,
            sample_height: height,

            quad_width,
            quad_height,

            quads,
        });
    }

    Ok(all_blocks)
}