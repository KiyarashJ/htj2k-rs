use dicom_object::DefaultDicomObject;

use crate::{errors::data_invalid::invalid_err, output::writer_type::WriterResult};

pub fn required_u16(
    object: &DefaultDicomObject,
    tag: dicom_core::Tag,
    name: &str,
) -> WriterResult<u16> {
    let element = object
        .get(tag)
        .ok_or_else(|| invalid_err(format!("Missing DICOM attribute: {name}")))?;

    Ok(element.to_int::<u16>()?)
}