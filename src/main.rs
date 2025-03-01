use clap::{Parser};

mod commands;
mod archive;

use commands::Commands::{Pack, Unpack};
use commands::Cli;


fn main() {

    let cli = Cli::parse();

    match cli.command {
        Pack { input, output } => {
            println!("Упаковка файлов {:?} в {}", input, output);
        }
        Unpack { archive, output_dir } => {
            println!("Распаковка {} в {}", archive, output_dir);
            // Тут вызываем функцию разархивирования
        }
    }



}
