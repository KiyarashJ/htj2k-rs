
use crate::{errors::data_invalid::invalid_err, output::{writer_type::WriterResult, siz_info_struct::SizInfo, read_16_be::read_u16_be, read_32_be::read_u32_be}};

pub fn parse_siz(codestream: &[u8]) -> WriterResult<SizInfo> {
    if codestream.len() < 45 {
        return Err(invalid_err("Codestream is too short"));
    }
    if codestream.get(0..2) != Some(&[0xFF, 0x4F][..]) {
        return Err(invalid_err("Invalid JPEG 2000 SOC marker"));
    }
    if codestream.get(2..4) != Some(&[0xFF, 0x51][..]) {
        return Err(invalid_err("JPEG 2000 SIZ marker not found"));
    }
    let siz_length = usize::from(read_u16_be(codestream, 4)?);
    if siz_length < 41 {
        return Err(invalid_err("Invalid SIZ segment length"));
    }
    let siz_end = 4usize
        .checked_add(siz_length)
        .ok_or_else(|| invalid_err("SIZ length overflow"))?;
    if siz_end > codestream.len() {
        return Err(invalid_err("Truncated SIZ segment"));
    }
    let x_size = read_u32_be(codestream, 8)?;
    let y_size = read_u32_be(codestream, 12)?;
    let x_offset = read_u32_be(codestream, 16)?;
    let y_offset = read_u32_be(codestream, 20)?;
    let width = x_size
        .checked_sub(x_offset)
        .ok_or_else(|| invalid_err("Invalid SIZ horizontal dimensions"))?;
    let height = y_size
        .checked_sub(y_offset)
        .ok_or_else(|| invalid_err("Invalid SIZ vertical dimensions"))?;
    if width == 0 || height == 0 {
        return Err(invalid_err("Invalid image dimensions in SIZ"));
    }
    let components = read_u16_be(codestream, 40)?;
    if components != 1 {
        return Err(invalid_err(
            "Current DICOM writer supports grayscale images only"
        ));
    }
    let component_offset = 42;
    let ssiz = *codestream
        .get(component_offset)
        .ok_or_else(|| invalid_err("Missing component SIZ information"))?;
    let x_sampling = codestream[component_offset + 1];
    let y_sampling = codestream[component_offset + 2];
    if x_sampling != 1 || y_sampling != 1 {
        return Err(invalid_err(
            "Subsampled components are not supported by this writer"
        ));
    }
    let bit_depth = (ssiz & 0x7F) + 1;
    let signed = (ssiz & 0x80) != 0;
    Ok(SizInfo {
        width,
        height,
        components,
        bit_depth,
        signed,
    })
}
