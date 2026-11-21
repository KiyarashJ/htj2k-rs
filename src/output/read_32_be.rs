use crate::{errors::data_invalid::invalid_err, output::writer_type::WriterResult};

pub fn read_u32_be(data: &[u8], offset: usize) -> WriterResult<u32> {
    let bytes = data
        .get(offset..offset + 4)
        .ok_or_else(|| invalid_err("Unexpected end of JPEG 2000 codestream"))?;

    Ok(u32::from_be_bytes([
        bytes[0], bytes[1], bytes[2], bytes[3],
    ]))
}