use crate::ht_cleanup::{magnitude_exp::magnitude_exponent, quad::Quad, quad_analysis_struct::QuadAnalysis};


pub fn analyze_quad(quad: &Quad) -> QuadAnalysis {

    let mut magnitudes = [0u64; 4];

    let mut signs = [false; 4];

    let mut exponents = [0u8; 4];

    let mut magsgn_values = [0u64; 4];

    let mut rho = 0u8;

    for i in 0..4 {

        let coefficient = quad[i];

        let magnitude = u64::from(coefficient.unsigned_abs());

        let negative = coefficient < 0;

        magnitudes[i] = magnitude;

        signs[i] = negative;

        exponents[i] = magnitude_exponent(magnitude);

        let significant = magnitude != 0;

        if significant {
            rho |= 1 << i;
        }
        if significant {

            magsgn_values[i] =
                2 * (magnitude - 1) + u64::from(negative);

        }
    }

    let gamma = if matches!(rho, 0 | 1 | 2 | 4 | 8) {
        0
    } else {
        1
    };
    QuadAnalysis {
        magnitudes,
        signs,
        exponents,
        rho,
        gamma,
        magsgn_values,
    }
}

