use std::io::prelude::*;
pub fn read_file<F>(file_path: String, mut handler: F) -> std::io::Result<()>
where
    F: FnMut(&[u8]),
{
    let file_name = std::path::Path::new(&file_path);
    let opened_file = match std::fs::File::open(&file_name) {
        Ok(file) => file,
        Err(e) => {
            print!("Error while opening file: {e}");
            std::process::exit(0)
        }
    };
    let mut reader = std::io::BufReader::with_capacity(8 * 1024 * 1024, opened_file);
    let mut buffer = vec![0u8; 8 * 1024 * 1024];
    loop {
        let n = reader.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        handler(&buffer[..n]);
    }
    Ok(())
}
