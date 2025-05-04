use {
    anyhow::Result, clap::Parser, cli::{Cli, CliCommand::{self, File}, FileXorMode::{Addr, Symbol, Whole}}, elf::{endian::AnyEndian, ElfBytes}, hex_utils::hex_decode, xoarers::{xor_whole, xor_with_addr, xor_with_sym}
};

mod hex_utils;
mod sym_parser;
mod xoarers;
mod cli;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let key_bytes = hex_decode(cli.key)?;

    let out = match cli.command {
        File { file, mode } => {
            let input_file = std::fs::read(&file).expect("Sorry, can't read your file;....");
            match mode {
                Some(Addr { start, dest }) => {
                    xor_with_addr(input_file, start, dest, key_bytes)?
                },
                Some(Symbol { name }) => {
                    let file_for_elf = input_file.clone();
                    let elf_bytes = ElfBytes::<AnyEndian>::minimal_parse(file_for_elf.as_slice())
                        .expect("elf parse failed");
                    xor_with_sym(input_file, elf_bytes, name, key_bytes)?
                },
                Some(Whole) => {
                    xor_whole(input_file, key_bytes)
                }
                None => unreachable!(),
            }
        }
        CliCommand::String { string } => {
            let string_bytes = string.into_bytes();
            xor_whole(string_bytes, key_bytes)
        }
    };

    if let Some(path) = cli.out {
        std::fs::write(path, out)?
    } else {
        if let Ok(str) = String::from_utf8(out.clone()) {
            println!("{}", str)
        } else {
            println!("{:x?}", out)
        }
    }
    Ok(())
}
