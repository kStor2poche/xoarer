use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about = "A xoring tool with support for symbol xoring")]
pub struct Cli {
    #[command(subcommand)]
    pub input_type: InputType,
    #[arg(short, long)]
    pub key: String,
    #[arg(short, long)]
    pub out: Option<String>
}

#[derive(Subcommand, Debug)]
pub enum InputType {
    File {
        #[arg()]
        file: String,
        #[command(subcommand)]
        mode: Option<FileXorMode>,
    },
    String {
        #[arg()]
        string: String,
    }
}

#[derive(Subcommand, Debug)]
pub enum FileXorMode {
    Addr {
        #[arg()]
        start: String,
        #[clap(flatten)]
        dest: AddrDest,
    },
    Symbol {
        name: String,
    },
    Whole,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
pub struct AddrDest {
        #[arg(short = 't', long = "to")]
        pub end: Option<String>,
        #[arg(short = 'f', long = "for")]
        pub length: Option<String>,
}
