// from https://github.com/rust-ctf/ctf-pwn/blob/main/src/unix/symbol.rs
use elf::{ElfBytes, ParseError, endian::AnyEndian, string_table::StringTable};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Symbol {
    pub name: String,
    pub shndx: u16,
    pub value: u64,
    pub size: u64,
}

impl Symbol {
    fn parse(value: &elf::symbol::Symbol, string_table: &StringTable) -> Result<Self, ParseError> {
        let name = match value.st_name {
            0 => String::new(),
            idx => string_table.get(idx as usize)?.to_string(),
        };
        Ok(Symbol {
            name,
            shndx: value.st_shndx,
            value: value.st_value,
            size: value.st_size,
        })
    }

    fn parse_table<'a>(
        table: &elf::symbol::SymbolTable<'a, AnyEndian>,
        string_table: &StringTable,
    ) -> Result<Vec<Self>, ParseError> {
        table
            .iter()
            .map(|s| Self::parse(&s, string_table))
            .collect()
    }

    pub(crate) fn parse_symbol_table(file: &ElfBytes<AnyEndian>) -> Result<Vec<Self>, ParseError> {
        match file.symbol_table()? {
            None => Ok(Vec::new()),
            Some((table, string_table)) => Self::parse_table(&table, &string_table),
        }
    }

    pub fn find(elf_bytes: &ElfBytes<AnyEndian>, sym_name: String) -> Option<Self> {
        let parsed = Symbol::parse_symbol_table(&elf_bytes).expect("couldn't parse symtab");
        let our_sym = parsed
            .into_iter()
            .filter(|s| s.name == sym_name)
            .collect::<Vec<_>>();

        our_sym.get(0).cloned()
    }
}
