use crate::{dicom_htj2k_writer::parse_siz, errors::data_invalid::invalid_err, output::{siz_info_struct::SizInfo, writer_type::WriterResult}};

pub fn validate_codestream(codestream: &[u8]) -> WriterResult<SizInfo> {
    if codestream.len() < 4 {
        return Err(invalid_err("Codestream is empty or too short"));
    }
    if !codestream.ends_with(&[0xFF, 0xD9]) {
        return Err(invalid_err("JPEG 2000 EOC marker is missing"));
    }
    parse_siz(codestream)
}