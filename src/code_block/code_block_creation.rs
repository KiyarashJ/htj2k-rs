use crate::{code_block::matrix_generator::generate, subbands::subbands_struct::Subbands};

pub fn codeblock_create(subbands : Subbands ) -> (Vec<Vec<Vec<i32>>>,Vec<Vec<Vec<i32>>>,Vec<Vec<Vec<i32>>>,Vec<Vec<Vec<i32>>>){
    let ll = subbands.ll;
    let lh = subbands.lh;
    let hl = subbands.hl;
    let hh = subbands.hh;
    let ll_code_block = generate(&ll);
    let lh_code_block = generate(&lh);
    let hl_code_block = generate(&hl);
    let hh_code_block = generate(&hh);

    (ll_code_block ,lh_code_block ,hl_code_block ,hh_code_block)
}