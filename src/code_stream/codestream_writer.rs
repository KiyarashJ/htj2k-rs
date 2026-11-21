use ndic_core::{Result};
use ndic_codestream::markers::{
    self,
    Cap,
    Cod,
    ComponentSiz,
    Siz,
    Sot,
    com_payload,
    plt_payload,
    push_segment,
    tlm_payload,
};
use ndic_codestream::quant::Quant;

use crate::code_stream::code_stream_config_struct::Codestreamconfig;
use crate::code_stream::segment_enum::MarkerTarget;
use crate::errors::invalid::invalid;
use super::packet_builder::EncodedPacket;


pub fn write_htj2k_codestream(
    config: &Codestreamconfig,
    packets: &[EncodedPacket],
) -> Result<Vec<u8>> {
    let width = config.width;
    let height = config.height;
    let bit_depth = config.bit_depth;
    let levels = config.decomposition_levels;
    if width == 0 || height == 0 {
        return Err(invalid("Image dimensions cannot be zero"));
    }
    if !(1..=38).contains(&bit_depth) {
        return Err(invalid("Bit depth must be between 1 and 38"));
    }
    if levels > 32 {
        return Err(invalid("Maximum decomposition levels is 32"));
    }
    if packets.len() != usize::from(levels) + 1 {
        return Err(invalid(
            "Packet count must equal decomposition levels + 1"
        ));
    }
    for (index, packet) in packets.iter().enumerate() {
        if usize::from(packet.resolution) != index {
            return Err(invalid(
                "Packets must be ordered by increasing resolution"
            ));
        }
        if packet.data.is_empty() {
            return Err(invalid(
                "Packet data cannot be empty"
            ));
        }
    }
    let cbw = config.code_block_width;
    let cbh = config.code_block_height;
    if !(4..=1024).contains(&cbw)
        || !(4..=1024).contains(&cbh)
        || !cbw.is_power_of_two()
        || !cbh.is_power_of_two()
        || cbw.checked_mul(cbh).is_none_or(|area| area > 4096)
    {
        return Err(invalid(
            "Invalid code-block dimensions"
        ));
    }
    let quant = Quant::reversible(
        levels,
        bit_depth,
        false,
    )?;
    let mut max_k = 0u8;
    for resolution in 0..=levels {
        if resolution == 0 {
            max_k = max_k.max(quant.k_max(0, 0));
        } else {
            for band in 1..=3 {
                max_k = max_k.max(
                    quant.k_max(resolution, band)
                );
            }
        }
    }
    if max_k > 30 {
        return Err(invalid(
            "K_max exceeds the HT 32-bit coder limit"
        ));
    }
    let mut body = Vec::new();
    let mut packet_lengths = Vec::with_capacity(packets.len());
    for packet in packets {
        let packet_len = u32::try_from(packet.data.len())
            .map_err(|_| invalid("Packet is too large"))?;

        packet_lengths.push(packet_len);
        body.extend_from_slice(&packet.data);
    }
    let plt_payload = plt_payload(0, &packet_lengths);
    let mut plt_segment = Vec::new();
    let psot = 12usize
        .checked_add(plt_segment.len())
        .and_then(|v| v.checked_add(2))
        .and_then(|v| v.checked_add(body.len()))
        .ok_or_else(|| invalid("Tile-part size overflow"))?;
    let psot_u32 = u32::try_from(psot)
        .map_err(|_| invalid("Tile-part exceeds u32 length"))?;
    let mut tile_part = Vec::with_capacity(psot);
    let sot = Sot {
        isot: 0,
        psot: psot_u32,
        tpsot: 0,
        tnsot: 1,
    };

    tile_part.extend_from_slice(&plt_segment);
    tile_part.extend_from_slice(&markers::SOD.to_be_bytes());
    tile_part.extend_from_slice(&body);
    if tile_part.len() != psot {
        return Err(invalid("Tile-part length calculation mismatch"));
    }
    let xsiz = u32::try_from(width)
        .map_err(|_| invalid("Image width exceeds u32"))?;
    let ysiz = u32::try_from(height)
        .map_err(|_| invalid("Image height exceeds u32"))?;
    let siz = Siz {
        rsiz: 0x4000,
        xsiz,
        ysiz,
        xosiz: 0,
        yosiz: 0,
        xtsiz: xsiz,
        ytsiz: ysiz,
        xtosiz: 0,
        ytosiz: 0,
        comps: vec![
            ComponentSiz {
                depth: bit_depth,
                signed: config.signed,
                xr: 1,
                yr: 1,
            }
        ],
    };
    let mut output = Vec::with_capacity(tile_part.len() + 256);
    output.extend_from_slice(&markers::SOC.to_be_bytes());
    
    let cbw_exp = (cbw.trailing_zeros() - 2) as u8;
    let cbh_exp = (cbh.trailing_zeros() - 2) as u8;
    let cod = Cod {
        scod: 0,
        progression: 2, 
        layers: 1,
        mct: 0,
        decomps: levels,
        cb_width_exp: cbw_exp,
        cb_height_exp: cbh_exp,
        cb_style: 0x40, 
        wavelet: 1,   
        precincts: Vec::new(),
    };
    let cap = Cap::new(false, quant.magb());
    let tlm = tlm_payload(&[psot_u32]);
    let file_markers = [
        (MarkerTarget::PltSegment, markers::PLT, &plt_payload),
        (MarkerTarget::TilePart, markers::SOT, &sot.to_bytes()),
        (MarkerTarget::MainHeader, markers::SIZ, &siz.to_bytes()),
        (MarkerTarget::MainHeader, markers::COD, &cod.to_bytes()), 
        (MarkerTarget::MainHeader, markers::QCD, &quant.to_bytes()),
        (MarkerTarget::MainHeader, markers::CAP, &cap.to_bytes()),
        (MarkerTarget::MainHeader, markers::COM, &com_payload("Rust HTJ2K DICOM Encoder")),
        (MarkerTarget::MainHeader, markers::TLM, &tlm)
    ];
    for (target, marker, payload) in &file_markers {
        let destination = match target {
            MarkerTarget::MainHeader => &mut output,
            MarkerTarget::TilePart => &mut tile_part,
            MarkerTarget::PltSegment => &mut plt_segment,
        };
        push_segment(destination, *marker, payload)?;
    }
    output.extend_from_slice(&tile_part);
    output.extend_from_slice(&markers::EOC.to_be_bytes());
    Ok(output)
}
