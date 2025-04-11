use {
    anyhow::{Result, anyhow},
    elf::{ElfBytes, endian::AnyEndian},
};

use crate::{
    hex_utils::{hex_bytes_to_usize, hex_decode},
    sym_parser::Symbol,
};

pub fn xor_with_sym(
    orig_path: String,
    mut input_file: Vec<u8>,
    elf_bytes: ElfBytes<AnyEndian>,
    sym_name: String,
    key: String,
) -> Result<()> {
    let key_bytes = hex_decode(key)?;

    // How can we get mapping between binary and loaded layouts? Or is it even needed?
    let symbol = Symbol::find(&elf_bytes, sym_name.clone())
        .ok_or(anyhow!("Symbol \"{}\" not found", sym_name))?;

    //println!("Gread, found symbol {:?}", symbol);
    input_file
        .iter_mut()
        .skip(symbol.value as usize)
        .enumerate()
        .for_each(|(i, byte)| {
            if i < symbol.size as usize {
                *byte ^= key_bytes[i % key_bytes.len()];
            }
        });

    Ok(std::fs::write(orig_path + "-xored", input_file)?)
}

pub fn xor_with_addr(
    orig_path: String,
    mut input_file: Vec<u8>,
    start_addr: String,
    len: String,
    key: String,
) -> Result<()> {
    let key_bytes = hex_decode(key)?;
    let start_addr = hex_bytes_to_usize(hex_decode(start_addr)?)?;
    let len: usize = if let Ok(n) = len.parse::<usize>() {
        Ok::<usize, anyhow::Error>(n)
    } else {
        let bytes = hex_decode(len)?;
        Ok(hex_bytes_to_usize(bytes)?)
    }?;

    input_file
        .iter_mut()
        .skip(start_addr)
        .enumerate()
        .for_each(|(i, byte)| {
            if i < len {
                *byte ^= key_bytes[i % key_bytes.len()];
            }
        });

    Ok(std::fs::write(orig_path + "-xored", input_file)?)
}
