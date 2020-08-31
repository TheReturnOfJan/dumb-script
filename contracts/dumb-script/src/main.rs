#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]

// Import from `core` instead of from `std` since we are in no-std mode
use core::result::Result;

// Import heap related library from `alloc`
// https://doc.rust-lang.org/alloc/index.html
use alloc::{vec, vec::Vec};

// Import CKB syscalls and structures
// https://nervosnetwork.github.io/ckb-std/riscv64imac-unknown-none-elf/doc/ckb_std/index.html
use ckb_std::{
    entry,
    default_alloc,
    debug,
    high_level::{load_script, load_tx_hash, load_witness_args},
    syscalls::{load_witness},
    error::SysError,
    ckb_types::{bytes::Bytes, prelude::*},
    ckb_constants::{Source},
};

use blake2b_rs::{Blake2bBuilder};

entry!(entry);
default_alloc!();

const CKB_HASH_PERSONALIZATION: &[u8] = b"ckb-default-hash";
pub fn blake2b(data: &[u8], dst: &mut [u8]) {
    let mut blake2b = Blake2bBuilder::new(dst.len()).personal(CKB_HASH_PERSONALIZATION).build();
    blake2b.update(data);
    blake2b.finalize(dst)
}

/// Program entry
fn entry() -> i8 {
    // Call main function and return error code
    match main() {
        Ok(_) => 0,
        Err(err) => err as i8,
    }
}

/// Error
#[repr(i8)]
enum Error {
    IndexOutOfBound = 1,
    ItemMissing,
    LengthNotEnough,
    Encoding,
    WrongWitness,
    // Add customized errors here...
}

impl From<SysError> for Error {
    fn from(err: SysError) -> Self {
        use SysError::*;
        match err {
            IndexOutOfBound => Self::IndexOutOfBound,
            ItemMissing => Self::ItemMissing,
            LengthNotEnough(_) => Self::LengthNotEnough,
            Encoding => Self::Encoding,
            WrongWitness => Self::WrongWitness,
            Unknown(err_code) => panic!("unexpected sys error {}", err_code),
        }
    }
}

fn main() -> Result<(), Error> {
    
    // let data : &[u8] = &([0, 1, 13, 5, 6][..]);
    // let mut hash = [0u8; 32];
    // blake2b(data, &mut hash);

    // let script = load_script()?;
    // let args: Bytes = script.args().unpack();
    // debug!("script args is {:?}", args);

    // let tx_hash = load_tx_hash()?;
    // debug!("tx hash is {:?}", tx_hash);

    // debug!("blake hash is {:?}", hash);
    // // debug!("blake hash is {:?}", &(hash[0..31]));
    // // debug!("blake hash is {:?}", &(hash[32..63]));

    // let _buf: Vec<_> = vec![0u8; 32];

    // Ok(())

    // get args
    let script = load_script()?;
    let args: Bytes = script.args().unpack();
    debug!("script args is {:?}", args);

    // get witness
    let mut wit_buf: [u8; 32] = [0; 32];
    load_witness(&mut wit_buf, 0, 0, Source::Input)?;
    let wit = Bytes::from(&wit_buf[..]);
    debug!("witness is {:?}", wit);

    // hash witness
    let mut wit_hash = [0u8; 32];
    blake2b(&wit_buf, &mut wit_hash);
    debug!("witness hash is {:?}", wit_hash);
    let wit_hash_b = Bytes::from(&wit_hash[..]);

    // for 
    
    if *args == *wit_hash_b {
        Ok(())
    } else {
        Err(Error::WrongWitness)
    }
}
