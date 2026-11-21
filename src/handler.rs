use crate::{
    code_block::multilevel_codeblock_encoder::encode_multilevel_to_packets, code_stream::{code_stream_config_struct::Codestreamconfig, codestream_writer::write_htj2k_codestream}, dc_shifting::shifter::shift, get_dcm_pixels::{
        r#enum::GetPixelsError, get_pixels::get_pixels,
    }, subbands::multilevel::multilevel_subband, wavelet::two_dimensional_array::to_2d,
    output::{default_output_path::default_output_path, write_htj2k_file::write_htj2k_dicom}
};

const DWT_LEVELS: u8 = 5;
const CODEBLOCK_WIDTH: usize = 64;
const CODEBLOCK_HEIGHT: usize = 64;

pub fn to_htj2k(file: &str) -> Result<(), GetPixelsError> {
    let output_path = default_output_path(file);

    let output = output_path.to_str().ok_or_else(|| {
        GetPixelsError::ProcessingError(
            "Output path is not valid UTF-8".to_string()
        )
    })?;

    handler_to(file, output)
}

pub fn handler_to(
    input_file: &str,
    output_file: &str,
) -> Result<(), GetPixelsError> {
    let dicom_data = get_pixels(input_file)?;
    let rows = usize::from(dicom_data.rows);
    let columns = usize::from(dicom_data.columns);
    let bits_stored = dicom_data.bits_stored;
    let is_signed = dicom_data.is_signed;
    let bit_depth = u8::try_from(bits_stored)
        .map_err(|_| {
            GetPixelsError::ProcessingError(
                "BitsStored cannot fit into u8".to_string()
            )
        })?;
    let shifted_pixels = shift(
        bits_stored,
        is_signed,
        dicom_data.data,
    )
    .map_err(|e| {
        GetPixelsError::ProcessingError(e.to_string())
    })?;
    let image = to_2d(
        &shifted_pixels,
        rows,
        columns,
    );
    let multilevel = multilevel_subband(
        &image,
        DWT_LEVELS,
    )
    .map_err(|e| {
        GetPixelsError::ProcessingError(e)
    })?;
    let packets = encode_multilevel_to_packets(
        &multilevel,
        bit_depth,
    )
    .map_err(|e| {
        GetPixelsError::ProcessingError(e.to_string())
    })?;
    let config = Codestreamconfig {
        width: columns,
        height: rows,
        bit_depth,
        signed: is_signed,
        decomposition_levels: DWT_LEVELS,
        code_block_width: CODEBLOCK_WIDTH,
        code_block_height: CODEBLOCK_HEIGHT,
    };

    let codestream = write_htj2k_codestream(
        &config,
        &packets,
    )
    .map_err(|e| {
        GetPixelsError::ProcessingError(e.to_string())
    })?;

    if codestream.is_empty() {
        return Err(GetPixelsError::ProcessingError(
            "Generated Codestream is empty".to_string()
        ));
    }
    write_htj2k_dicom(
        input_file,
        &codestream,
        output_file,
    )
    .map_err(|e| {
        GetPixelsError::ProcessingError(e.to_string())
    })?;
    Ok(())
}
