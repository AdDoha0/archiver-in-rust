use std::{fs::File, io::Result};

use std::io::{Read, Write};
use std::path::Path;
use std::io;
use zip::{ZipWriter, write::FileOptions};


pub  fn pack_files(input_files: &[String], output_path: &str) -> Result<()> {

    let file = File::create(output_path)?;
    let mut zip = ZipWriter::new(file);

    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o644);


    for input_path in input_files {
        let path = Path::new(input_path);
        let name = path.file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid file name"))?;

        let mut iput_file = File::open(path)?;
        zip.start_file(name, options)?;
        io::copy(&mut iput_file, &mut zip)?;
    }

    zip.finish()?;s
    println!("Archive created successfully: {}", output_path);

    Ok(())
}




mod test {
    use super::*;
    use std::fs;
    use std::io::Write;

    #[test]
    fn test_pack_files() -> Result<()> {
        let test_dir = "test_files";
        fs::create_dir_all(test_dir)?;

        // Создаём файлы разных типов
        let text_file = format!("{}/text.txt", test_dir);
        let binary_file = format!("{}/binary.dat", test_dir);

        // Записываем текстовые данные
        fs::write(&text_file, "Text content")?;

        // Записываем бинарные данные
        let mut binary_data = vec![0; 100];
        rand::thread_rng().fill(&mut binary_data);
        fs::write(&binary_file, &binary_data)?;


        Ok(())
    }
}












