use dicom_dictionary_std::tags;
use dicom_object::DefaultDicomObject;

use crate::{errors::data_invalid::invalid_err, output::{get_dicom_us::required_u16, siz_info_struct::SizInfo, writer_type::WriterResult}};

pub fn validate_dicom_metadata(
    object: &DefaultDicomObject,
    siz: &SizInfo,
) -> WriterResult<()> {
    let rows = required_u16(object, tags::ROWS, "Rows")?;
    let columns = required_u16(object, tags::COLUMNS, "Columns")?;

    let samples_per_pixel =
        required_u16(object, tags::SAMPLES_PER_PIXEL, "SamplesPerPixel")?;

    let bits_allocated =
        required_u16(object, tags::BITS_ALLOCATED, "BitsAllocated")?;

    let bits_stored =
        required_u16(object, tags::BITS_STORED, "BitsStored")?;

    let high_bit =
        required_u16(object, tags::HIGH_BIT, "HighBit")?;

    let pixel_representation =
        required_u16(object, tags::PIXEL_REPRESENTATION, "PixelRepresentation")?;
    let number_of_frames = match object.get(tags::NUMBER_OF_FRAMES) {
        Some(element) => element.to_str()?.trim().parse::<u32>()?,
        None => 1,
    };

    if number_of_frames != 1 {
        return Err(invalid_err(
            "Current writer supports single-frame DICOM images only"
        ));
    }

    if samples_per_pixel != 1 || siz.components != 1 {
        return Err(invalid_err(
            "Current writer supports grayscale images only"
        ));
    }

    let photometric = object
        .get(tags::PHOTOMETRIC_INTERPRETATION)
        .ok_or_else(|| invalid_err("Missing PhotometricInterpretation"))?
        .to_str()?;

    if photometric.trim() != "MONOCHROME1"
        && photometric.trim() != "MONOCHROME2"
    {
        return Err(invalid_err(
            "Only MONOCHROME1 and MONOCHROME2 are supported"
        ));
    }

    if ![8, 16, 24, 32, 40].contains(&bits_allocated) {
        return Err(invalid_err("Unsupported BitsAllocated for HTJ2K"));
    }

    if !(1..=38).contains(&bits_stored) {
        return Err(invalid_err("Invalid BitsStored for HTJ2K"));
    }

    if high_bit != bits_stored - 1 {
        return Err(invalid_err(
            "HighBit must equal BitsStored - 1"
        ));
    }

    if pixel_representation > 1 {
        return Err(invalid_err("Invalid PixelRepresentation"));
    }

    if u32::from(rows) != siz.height {
        return Err(invalid_err("DICOM Rows does not match Codestream height"));
    }

    if u32::from(columns) != siz.width {
        return Err(invalid_err("DICOM Columns does not match Codestream width"));
    }

    if u8::try_from(bits_stored).ok() != Some(siz.bit_depth) {
        return Err(invalid_err("DICOM BitsStored does not match Codestream precision"));
    }

    let dicom_signed = pixel_representation == 1;

    if dicom_signed != siz.signed {
        return Err(invalid_err(
            "DICOM PixelRepresentation does not match Codestream signedness"
        ));
    }

    Ok(())
}