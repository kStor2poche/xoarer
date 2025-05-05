use {
    anyhow::{Result, anyhow},
    elf::{ElfBytes, endian::AnyEndian},
};

use crate::{
    cli::AddrDest, hex_utils::{hex_bytes_to_usize, hex_decode}, sym_parser::Symbol
};

pub fn xor_with_sym(
    mut input_file: Vec<u8>,
    elf_bytes: ElfBytes<AnyEndian>,
    sym_name: String,
    key: Vec<u8>,
) -> Result<Vec<u8>> {
    // How can we get mapping between binary and loaded layouts? Or is it even needed?
    let symbol = Symbol::find(&elf_bytes, sym_name.clone())
        .ok_or(anyhow!("Symbol \"{}\" not found", sym_name))?;

    let file_offset = symbol
        .get_file_offset(&elf_bytes)
        .ok_or(anyhow!("Couldn't get file offset"))?;
    //println!("Gread, found symbol {:?}", symbol);
    input_file
        .iter_mut()
        .skip(file_offset as usize)
        .enumerate()
        .for_each(|(i, byte)| {
            if i < symbol.size as usize {
                *byte ^= key[i % key.len()];
            }
        });

    Ok(input_file)
}

pub fn xor_with_addr(
    mut input_file: Vec<u8>,
    start_addr: String,
    dest: AddrDest,
    key: Vec<u8>,
) -> Result<Vec<u8>> {
    let start_addr = hex_bytes_to_usize(hex_decode(start_addr)?)?;
    let len: usize = match dest {
        AddrDest { end: None, length: Some(len) } => {
            if let Ok(n) = len.parse::<usize>() {
                Ok::<usize, anyhow::Error>(n)
            } else {
                let bytes = hex_decode(len)?;
                Ok(hex_bytes_to_usize(bytes)?)
            }?
        },
        AddrDest { end: Some(end), length: None } => {
            let end_parsed = if let Ok(n) = end.parse::<usize>() {
                Ok::<usize, anyhow::Error>(n)
            } else {
                let bytes = hex_decode(end)?;
                Ok(hex_bytes_to_usize(bytes)?)
            }?;
            end_parsed - start_addr
        }
        _ => unreachable!(),
    };

    input_file
        .iter_mut()
        .skip(start_addr)
        .enumerate()
        .take(len)
        .for_each(|(i, byte)| {
            *byte ^= key[i % key.len()];
        });

    Ok(input_file)
}

pub fn xor_whole(mut to_xor: Vec<u8>, key: Vec<u8>) -> Vec<u8> {
    to_xor.iter_mut()
        .enumerate()
        .for_each(|(i, byte)| {
            *byte ^= key[i % key.len()];
        });
    to_xor
}
