use crate::{errors::data_invalid::invalid_err, output::writer_type::WriterResult};

pub fn read_u16_be(data: &[u8], offset: usize) -> WriterResult<u16> {
    let bytes = data
        .get(offset..offset + 2)
        .ok_or_else(|| invalid_err("Unexpected end of JPEG 2000 codestream"))?;

    Ok(u16::from_be_bytes([bytes[0], bytes[1]]))
}