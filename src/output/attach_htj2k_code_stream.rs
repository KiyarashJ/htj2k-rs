use dicom_core::{DataElement, VR, value::Value};
use dicom_dictionary_std::{tags, uids};
use dicom_object::{DefaultDicomObject, InMemDicomObject, mem::InMemElement};
use dicom_pixeldata::encapsulation::encapsulate_single_frame;

use crate::{errors::data_invalid::invalid_err, output::{validate_code_stream::validate_codestream, validate_dicom_mt::validate_dicom_metadata, writer_type::WriterResult}};

pub fn attach_htj2k_codestream(
    object: &mut DefaultDicomObject,
    codestream: &[u8],
) -> WriterResult<()> {
    let siz = validate_codestream(codestream)?;
    validate_dicom_metadata(object, &siz)?;
    let encapsulated = encapsulate_single_frame(
    codestream.to_vec(),
    0,
);
let pixel_data: Value<InMemDicomObject> = match encapsulated {
    Value::PixelSequence(sequence) => {
        Value::PixelSequence(sequence)
    }
    _ => {
        return Err(invalid_err(
            "Failed to encapsulate HTJ2K Pixel Data"
        ));
    }
};
let pixel_element: InMemElement = DataElement::new(
    tags::PIXEL_DATA,
    VR::OB,
    pixel_data,
);

object.put(pixel_element);
    object.update_meta(|meta| {
        meta.transfer_syntax = uids::HTJ2K_LOSSLESS.to_string();
    });

    Ok(())
}