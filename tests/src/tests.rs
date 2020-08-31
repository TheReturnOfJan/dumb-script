use super::*;
use ckb_testtool::context::Context;
use ckb_tool::ckb_types::{
    bytes::Bytes,
    core::TransactionBuilder,
    packed::*,
    prelude::*,
};
use blake2b_rs::{Blake2bBuilder};

const MAX_CYCLES: u64 = 10_000_000;

const CKB_HASH_PERSONALIZATION: &[u8] = b"ckb-default-hash";
pub fn blake2b(data: &[u8], dst: &mut [u8]) {
    let mut blake2b = Blake2bBuilder::new(dst.len()).personal(CKB_HASH_PERSONALIZATION).build();
    blake2b.update(data);
    blake2b.finalize(dst)
}

fn copy_slice(dst: &mut [u8], src: &[u8]) -> usize {
    let mut c = 0;
    for (d, s) in dst.iter_mut().zip(src.iter()) {
        *d = *s;
        c += 1;
    }
    c 
}

pub fn pad(arr: &[u8]) -> [u8; 32] {
    let mut paded = [0u8; 32];
    copy_slice(&mut paded, arr);
    paded
}

#[test]
fn test_basic() {
    // deploy contract
    let mut context = Context::default();
    let contract_bin: Bytes = Loader::default().load_binary("dumb-script");
    let contract_out_point = context.deploy_contract(contract_bin);

    let mut hash = [0u8; 32];
    blake2b(&pad(b"HEHE")[..], &mut hash);
    
    // prepare scripts
    let lock_script = context
        .build_script(&contract_out_point, Bytes::from(hash.to_vec()))
        .expect("script");
    let lock_script_dep = CellDep::new_builder()
        .out_point(contract_out_point)
        .build();

    // prepare cells
    let input_out_point = context.create_cell(
        CellOutput::new_builder()
            .capacity(1000u64.pack())
            .lock(lock_script.clone())
            .build(),
        Bytes::new(),
    );
    let input = CellInput::new_builder()
        .previous_output(input_out_point)
        .build();
    let outputs = vec![
        CellOutput::new_builder()
            .capacity(500u64.pack())
            .lock(lock_script.clone())
            .build(),
        CellOutput::new_builder()
            .capacity(500u64.pack())
            .lock(lock_script)
            .build(),
    ];

    let outputs_data = vec![Bytes::new(); 2];

    let witnesses = vec![Bytes::from("HEHEH"); 1];

    // build transaction
    let tx = TransactionBuilder::default()
        .input(input)
        .outputs(outputs)
        .outputs_data(outputs_data.pack())
        .witnesses(witnesses.pack())
        .cell_dep(lock_script_dep)
        .build();
    let tx = context.complete_tx(tx);

    // run
    let cycles = context
        .verify_tx(&tx, MAX_CYCLES)
        .expect("pass verification");
     println!("consume cycles: {}", cycles);
}
