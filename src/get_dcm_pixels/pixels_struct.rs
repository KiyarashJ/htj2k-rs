#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct DICOMFILE {
    pub rows: u16,
    pub columns: u16,
    pub bits_allocated: u16,
    pub bits_stored: u16,
    pub high_bit: u16,
    pub is_signed: bool,
    pub samples_per_pixel: u16,
    pub photometric_interpretation: String,
    pub data: Vec<i32>,
}
