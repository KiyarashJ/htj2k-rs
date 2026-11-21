use std::{path::Path};
use dicom_object::{open_file};

use crate::output::{attach_htj2k_code_stream::attach_htj2k_codestream, writer_type::WriterResult};

pub fn write_htj2k_dicom(
    input_path: impl AsRef<Path>,
    codestream: &[u8],
    output_path: impl AsRef<Path>,
) -> WriterResult<()> {
    let mut object = open_file(input_path)?;
    attach_htj2k_codestream(
        &mut object,
        codestream,
    )?;
    object.write_to_file(output_path)?;
    Ok(())
}
