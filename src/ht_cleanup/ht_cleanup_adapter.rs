use ndic_core::{Result};

use ndic_codestream::{
    quant::Quant,
};
use crate::errors::invalid::invalid;


pub fn k_max_for_band(
    bit_depth: u8,
    decomposition_levels: u8,
    resolution: u8,
    band: u8,
) -> Result<u8> {
    if resolution > decomposition_levels {
        return Err(invalid("Resolution exceeds decomposition levels"));
    }

    if resolution == 0 {
        if band != 0 {
            return Err(invalid("Resolution zero only contains LL band"));
        }
    } else if !(1..=3).contains(&band) {
        return Err(invalid("Detail band must be 1, 2, or 3"));
    }

    let quant = Quant::reversible(
        decomposition_levels,
        bit_depth,
        false,
    )?;

    let k_max = quant.k_max(resolution, band);
    if !(1..=30).contains(&k_max) {
        return Err(invalid("K_max is outside HT 32-bit coder range"));
    }

    Ok(k_max)
}


