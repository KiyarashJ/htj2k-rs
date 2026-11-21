pub mod handler;
pub mod get_dcm_pixels;
pub mod dc_shifting;
pub mod wavelet;
pub mod subbands;
pub mod code_block;
pub mod ht_cleanup;
pub mod code_stream;
pub mod dicom_htj2k_writer;
pub mod errors;
pub mod output;

use crate::handler::to_htj2k;



fn main() {
    let file = r"E:\RUST_PROJECTS\HTJTWOK\htj2k\instance-0067.dcm";
    let _ = to_htj2k(file);
}
