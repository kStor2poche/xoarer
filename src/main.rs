use anyhow::{Result, anyhow};
use elf::{ElfBytes, endian::AnyEndian};
use std::env::args;
use sym_parser::Symbol;

mod sym_parser;

fn usage() {
    println!(
        "Usage: xoarerv2 [file] [mode] [key]\nWhere mode can be one of:\n    -symbol [sym_name]\n    -addr [start_addr] [len]\n\naddress and key need to be in hex format, len may be in decimal or hexadecimal (needs 0x prefix then)."
    )
}

fn hex_decode(s: String) -> Result<Vec<u8>> {
    let s = if let Some("0x") = s.get(0..2) {
        s.get(2..).expect("empty s (only got 0x prefix)")
    } else {
        s.as_str()
    };
    Ok(hex::decode(s)?)
}

fn hex_bytes_to_usize(mut bytes: Vec<u8>) -> Result<usize> {
    if bytes.len() > 8 {
        return Err(anyhow!("no can do len > 8 bytes"));
    }
    while bytes.len() != 8 {
        bytes.insert(0, 0);
    }
    Ok(usize::from_be_bytes([
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
    ]))
}

fn xor_with_sym(
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

fn xor_with_addr(
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

fn main() -> Result<()> {
    let args = args();
    if args.len() <= 1 {
        usage();
        return Err(anyhow!("No args? that's tubad!"));
    }
    let mut args = args.skip(1); // skip argv[0]
    let orig_path;
    let input_file = if let Some(path) = args.next() {
        orig_path = path;
        Ok::<_, anyhow::Error>(std::fs::read(&orig_path).expect("Sorry, can't read your file;...."))
    } else {
        usage();
        return Err(anyhow!("No file??"));
    }?;

    if let Some(arg) = args.next() {
        match arg.as_str() {
            "-symbol" | "-s" => {
                let file_for_elf = input_file.clone();
                let elf_bytes = ElfBytes::<AnyEndian>::minimal_parse(file_for_elf.as_slice())
                    .expect("elf parse failed");
                let sym_name = args.next().expect("symbol name arg not found");
                let key = args.next().expect("need a key, but don't have one >:(");
                xor_with_sym(orig_path, input_file, elf_bytes, sym_name, key)?
            }
            "-addr" | "-a" => {
                let start_addr = args.next().expect("start addr arg not found");
                let len = args.next().expect("len arg not found");
                let key = args.next().expect("need a key, but don't have one >:(");
                xor_with_addr(orig_path, input_file, start_addr, len, key)?
            }
            unknown => {
                usage();
                return Err(anyhow!("unknown option \"{}\"", unknown));
            }
        }
    } else {
        return Err(anyhow!("idk, don't ask me..."));
    }

    Ok(())
}
