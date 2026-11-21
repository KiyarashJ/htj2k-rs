
#[cfg(test)]
mod tests {
    use crate::code_stream::code_stream_config_struct::Codestreamconfig;
use crate::code_stream::codestream_writer::write_htj2k_codestream;

    use ndic_codestream::packet::{
        BandBlocks,
        CodedBlock,
    };

    use ndic_codestream::quant::Quant;

    use ndic_codestream::reader::Codestream;

    use ndic_htj2k::{
        coeff_to_sign_magnitude,
        encode_block,
    };

    use super::super::packet_builder::build_packets;

    #[test]
    fn complete_codestream_round_trip() {
        let width = 8usize;
        let height = 8usize;
        let bit_depth = 8u8;

        // پیکسل‌های خام unsigned
        let samples: Vec<i32> = (0..width * height)
            .map(|i| ((i * 37) % 256) as i32)
            .collect();

        // DC Shift برای ورودی unsigned
        let coefficients: Vec<i32> = samples
            .iter()
            .map(|&value| value - 128)
            .collect();

        let quant = Quant::reversible(
            0,
            bit_depth,
            false,
        ).unwrap();

        let k_max = quant.k_max(0, 0);

        let shift = 31 - u32::from(k_max);

        let sign_magnitude: Vec<u32> = coefficients
            .iter()
            .map(|&value| {
                coeff_to_sign_magnitude(value, shift)
            })
            .collect();

        let data = encode_block(
            &sign_magnitude,
            width,
            height,
            width,
            u32::from(k_max) - 1,
        ).unwrap();

        let coded_block = CodedBlock {
            missing_msbs: k_max - 1,
            num_passes: 1,
            len_cleanup: data.len() as u32,
            len_refinement: 0,
            data,
        };

        // یک Resolution با یک LL Band
        let ll = BandBlocks {
            nx: 1,
            ny: 1,
            blocks: vec![Some(coded_block)],
        };

        let packets = build_packets(&[
            vec![ll]
        ]).unwrap();

        let config = Codestreamconfig {
            width,
            height,
            bit_depth,
            signed: false,
            decomposition_levels: 0,
            code_block_width: 64,
            code_block_height: 64,
        };

        let codestream = write_htj2k_codestream(
            &config,
            &packets,
        ).unwrap();

        // Parse با Reader کتابخانه
        let parsed = Codestream::parse(&codestream)
            .expect("Codestream parsing failed");

        // Decode کامل
        let decoded = parsed.decode()
            .expect("Codestream decoding failed");

        assert_eq!(decoded.width, width);
        assert_eq!(decoded.height, height);
        assert_eq!(decoded.comps.len(), 1);

        // تست Lossless
        assert_eq!(
            decoded.comps[0],
            samples,
            "Decoded pixels differ from original pixels"
        );
    }
}
