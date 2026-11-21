
use std::error::Error as StdError;
use std::fmt;

use dicom_object::ReadError;
use dicom_pixeldata::Error as PixelDataError;

#[derive(Debug)]
pub enum GetPixelsError {
    ReadError(ReadError),

    InnerError(PixelDataError),

    InvalidPixelData(String),

    ProcessingError(String),
}

impl fmt::Display for GetPixelsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReadError(e) => {
                write!(f, "DICOM reading error: {e}")
            }

            Self::InnerError(e) => {
                write!(f, "DICOM pixel decoding error: {e}")
            }

            Self::InvalidPixelData(message) => {
                write!(f, "Invalid DICOM pixel data: {message}")
            }

            Self::ProcessingError(message) => {
                write!(f, "Image processing error: {message}")
            }
        }
    }
}

impl StdError for GetPixelsError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::ReadError(e) => Some(e),
            Self::InnerError(e) => Some(e),
            Self::InvalidPixelData(_) => None,
            Self::ProcessingError(_) => None,
        }
    }
}
