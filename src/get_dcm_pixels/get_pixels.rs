use dicom_object::open_file;
use dicom_pixeldata::{
    PixelDecoder,
    PixelRepresentation,
};
use crate::get_dcm_pixels::{
    decode_samples::decode_samples, r#enum::GetPixelsError, pixels_struct::DICOMFILE,
};

pub fn get_pixels(
    file: &str,
) -> Result<DICOMFILE, GetPixelsError> {

    let dicom_object = open_file(file)
        .map_err(GetPixelsError::ReadError)?;
    let image = dicom_object
        .decode_pixel_data()
        .map_err(GetPixelsError::InnerError)?;
    if image.number_of_frames() != 1 {
        return Err(GetPixelsError::InvalidPixelData(
            format!(
                "Only single-frame images are currently supported. Found {} frames.",
                image.number_of_frames()
            ),
        ));
    }
    if image.samples_per_pixel() != 1 {
        return Err(GetPixelsError::InvalidPixelData(
            format!(
                "Only grayscale images are supported. SamplesPerPixel={}",
                image.samples_per_pixel()
            ),
        ));
    }
    if !image.photometric_interpretation().is_monochrome() {
        return Err(GetPixelsError::InvalidPixelData(
            format!(
                "Unsupported PhotometricInterpretation: {}",
                image.photometric_interpretation()
            ),
        ));
    }
    let rows = u16::try_from(image.rows())
        .map_err(|_| {
            GetPixelsError::InvalidPixelData(
                "Rows exceeds u16 range".to_string()
            )
        })?;
    let columns = u16::try_from(image.columns())
        .map_err(|_| {
            GetPixelsError::InvalidPixelData(
                "Columns exceeds u16 range".to_string()
            )
        })?;
    if rows == 0 || columns == 0 {
        return Err(GetPixelsError::InvalidPixelData(
            "Image dimensions cannot be zero".to_string()
        ));
    }
    let bits_allocated = image.bits_allocated();
    let bits_stored = image.bits_stored();
    let high_bit = image.high_bit();
    if !matches!(bits_allocated, 8 | 16) {
        return Err(GetPixelsError::InvalidPixelData(
            format!(
                "Unsupported BitsAllocated: {bits_allocated}"
            ),
        ));
    }
    if bits_stored == 0 || bits_stored > bits_allocated {
        return Err(GetPixelsError::InvalidPixelData(
            "Invalid BitsStored value".to_string()
        ));
    }
    if high_bit != bits_stored - 1 {
        return Err(GetPixelsError::InvalidPixelData(
            format!(
                "HighBit must be BitsStored - 1. Found HighBit={high_bit}, BitsStored={bits_stored}"
            ),
        ));
    }
    let is_signed = matches!(
        image.pixel_representation(),
        PixelRepresentation::Signed
    );
    let raw_frame = image
        .frame_data(0)
        .map_err(GetPixelsError::InnerError)?;
    let data = decode_samples(
        raw_frame,
        bits_allocated,
        bits_stored,
        is_signed,
    )
    .map_err(GetPixelsError::InvalidPixelData)?;
    let expected_pixels = usize::from(rows)
        .checked_mul(usize::from(columns))
        .ok_or_else(|| {
            GetPixelsError::InvalidPixelData(
                "Image pixel count overflow".to_string()
            )
        })?;
    if data.len() != expected_pixels {
        return Err(GetPixelsError::InvalidPixelData(
            format!(
                "Pixel count mismatch: expected {expected_pixels}, got {}",
                data.len()
            ),
        ));
    }
    Ok(DICOMFILE {
        rows,
        columns,
        bits_allocated,
        bits_stored,
        high_bit,
        is_signed,
        samples_per_pixel: image.samples_per_pixel(),
        photometric_interpretation:
            image.photometric_interpretation().as_str().to_string(),
        data,
    })
}
