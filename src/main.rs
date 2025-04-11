use {
    anyhow::{Result, anyhow},
    elf::{ElfBytes, endian::AnyEndian},
    std::env::args,
    xoarers::{xor_with_addr, xor_with_sym},
};

mod hex_utils;
mod sym_parser;
mod xoarers;

fn usage() {
    println!(
        "Usage: xoarerv2 [file] [mode] [key]\nWhere mode can be one of:\n    -symbol [sym_name]\n    -addr [start_addr] [len]\n\naddress and key need to be in hex format, len may be in decimal or hexadecimal (needs 0x prefix then)."
    )
}

// TODO: clap rework
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
