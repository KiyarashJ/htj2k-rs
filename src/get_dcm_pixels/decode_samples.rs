use crate::get_dcm_pixels::decode_sample::decode_sample;

pub fn decode_samples(
    raw_data: &[u8],
    bits_allocated: u16,
    bits_stored: u16,
    is_signed: bool,
) -> Result<Vec<i32>, String> {
    if bits_stored == 0 || bits_stored > bits_allocated {
        return Err(format!(
            "Invalid BitsStored={bits_stored} for BitsAllocated={bits_allocated}"
        ));
    }

    if !matches!(bits_allocated, 8 | 16) {
        return Err(format!(
            "Unsupported BitsAllocated: {bits_allocated}"
        ));
    }

    let mut samples = Vec::new();

    match bits_allocated {
        8 => {
            samples.reserve(raw_data.len());

            for &byte in raw_data {
                let value = decode_sample(
                    u32::from(byte),
                    bits_stored,
                    is_signed,
                );

                samples.push(value);
            }
        }

        16 => {
            if raw_data.len() % 2 != 0 {
                return Err(
                    "16-bit Pixel Data has an odd byte count".to_string()
                );
            }

            samples.reserve(raw_data.len() / 2);

            for bytes in raw_data.chunks_exact(2) {
                let raw_value = u16::from_ne_bytes([
                    bytes[0],
                    bytes[1],
                ]);

                let value = decode_sample(
                    u32::from(raw_value),
                    bits_stored,
                    is_signed,
                );

                samples.push(value);
            }
        }

        _ => unreachable!(),
    }

    Ok(samples)
}