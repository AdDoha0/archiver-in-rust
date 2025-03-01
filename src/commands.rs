
use clap::{Parser, Subcommand,};


/// Основная структура команд CLI
#[derive(Parser)]
#[command(name = "myarchiver", version = "1.0", about = "Простой CLI-архиватор")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}


/// Перечисление доступных команд
#[derive(Subcommand)]
pub enum Commands {
    /// Упаковать файлы в архив
    Pack {
        /// Путь к файлам, которые нужно упаковать
        #[arg(required = true)]
        input: Vec<String>,

        /// Имя выходного архива
        #[arg(short, long, default_value = "archive.zip")]
        output: String,
    },

    /// Распаковать архив
    Unpack {
        /// Имя архива для распаковки
        #[arg(required = true)]
        archive: String,

        /// Папка, куда извлекать файлы
        #[arg(short, long, default_value = "./extracted")]
        output_dir: String,
    },
}

















