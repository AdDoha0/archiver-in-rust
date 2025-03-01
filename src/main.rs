use clap::{Parser};

mod commands;
mod archive;

use commands::Commands::{Pack, Unpack};
use commands::Cli;


// cargo run -- pack input.txt output.zip
// cargo run -- unpack output.zip ./output_dir

fn main() {
    if let Err(e) = start() {
        eprintln!("Ошибка: {}", e);
        std::process::exit(1);
    }
}


fn start() -> Result<(), clap::Error> {
    let cli = Cli::try_parse()?;

    match cli.command {
        Pack { input, output } => {
            println!("Упаковка файлов {:?} в {}", input, output);
        }
        Unpack { archive, output_dir } => {
            println!("Распаковка {} в {}", archive, output_dir);
        }
    }

    Ok(())
}
