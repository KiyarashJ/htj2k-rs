
use ndic_core::{Error, Result};
use ndic_codestream::packet::{write_packet, BandBlocks};

#[derive(Debug, Clone)]
pub struct EncodedPacket {
    pub resolution: u8,
    pub data: Vec<u8>,
}

fn invalid(message: impl Into<String>) -> Error {
    Error::InvalidArgument {
        message: message.into(),
    }
}

/// بررسی صحت ساختار BandBlocks
fn validate_band(band: &BandBlocks) -> Result<()> {
    let expected = band
        .nx
        .checked_mul(band.ny)
        .ok_or_else(|| invalid("Band block count overflow"))?;

    if band.blocks.len() != expected {
        return Err(invalid(format!(
            "Invalid BandBlocks: expected {expected} blocks, got {}",
            band.blocks.len()
        )));
    }

    for (index, block) in band.blocks.iter().enumerate() {
        let Some(block) = block else {
            continue;
        };

        if block.num_passes != 1 {
            return Err(invalid(format!(
                "Block {index}: only one cleanup pass is supported"
            )));
        }

        if block.len_refinement != 0 {
            return Err(invalid(format!(
                "Block {index}: refinement data is not supported"
            )));
        }

        if block.data.is_empty() {
            return Err(invalid(format!(
                "Block {index}: cleanup segment is empty"
            )));
        }

        if block.len_cleanup as usize != block.data.len() {
            return Err(invalid(format!(
                "Block {index}: cleanup length does not match data length"
            )));
        }
    }

    Ok(())
}

/// ساخت Packetهای یک Component.
///
/// ورودی:
/// resolutions[0] = [LL]
/// resolutions[1] = [HL, LH, HH]
/// resolutions[2] = [HL, LH, HH]
/// ...
///
/// هر BandBlocks مربوط به یک Precinct است.
/// در تنظیمات فعلی، هر Resolution یک Precinct دارد.
pub fn build_packets(
    resolutions: &[Vec<BandBlocks>],
) -> Result<Vec<EncodedPacket>> {
    if resolutions.is_empty() {
        return Err(invalid("At least one resolution is required"));
    }

    if resolutions.len() > 33 {
        return Err(invalid("Too many resolutions"));
    }

    let mut packets = Vec::with_capacity(resolutions.len());

    for (resolution, bands) in resolutions.iter().enumerate() {
        let expected_bands = if resolution == 0 { 1 } else { 3 };

        if bands.len() != expected_bands {
            return Err(invalid(format!(
                "Resolution {resolution}: expected {expected_bands} bands, got {}",
                bands.len()
            )));
        }

        for band in bands {
            validate_band(band)?;
        }

        // ترتیب bands همان ترتیب استاندارد Packet است.
        let data = write_packet(bands);

        if data.is_empty() {
            return Err(invalid(format!(
                "Packet generation failed at resolution {resolution}"
            )));
        }

        packets.push(EncodedPacket {
            resolution: resolution as u8,
            data,
        });
    }

    Ok(packets)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndic_codestream::packet::CodedBlock;

    fn dummy_block() -> CodedBlock {
        CodedBlock {
            missing_msbs: 3,
            num_passes: 1,
            len_cleanup: 2,
            len_refinement: 0,
            data: vec![0x12, 0x34],
        }
    }

    #[test]
    fn builds_single_ll_packet() {
        let ll = BandBlocks {
            nx: 1,
            ny: 1,
            blocks: vec![Some(dummy_block())],
        };

        let result = build_packets(&[vec![ll]]).unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].resolution, 0);
        assert!(!result[0].data.is_empty());
    }

    #[test]
    fn rejects_wrong_band_count() {
        let ll = BandBlocks {
            nx: 1,
            ny: 1,
            blocks: vec![None],
        };

        let hl = ll.clone();

        assert!(
            build_packets(&[vec![ll, hl]]).is_err()
        );
    }

    #[test]
    fn rejects_invalid_block_grid() {
        let invalid_band = BandBlocks {
            nx: 2,
            ny: 2,
            blocks: vec![None],
        };

        assert!(
            build_packets(&[vec![invalid_band]]).is_err()
        );
    }
}
