use colored::*;
use goblin::Object;

pub fn convert_to_string(file: &[u8]) {
    let ascii_bytes: Vec<u8> = file
        .iter()
        .filter(|b| (32..=126).contains(*b))
        .copied() // &u8 -> u8
        .collect();

    match String::from_utf8(ascii_bytes) {
        Ok(text) => print!("{}", text.truecolor(255, 68, 68)),
        Err(e) => {
            println!("{e}");
        }
    }
    println!("")
}
pub fn detect_file_format(buffer: &[u8]) {
    let text = if buffer.starts_with(&[0x7F, b'E', b'L', b'F']) {
        "ELF Executable (Linux Binary)"
    } else if buffer.starts_with(&[0x89, b'P', b'N', b'G']) {
        "PNG Image"
    } else if buffer.starts_with(&[0xFF, 0xD8, 0xFF]) {
        "JPEG Image"
    } else if buffer.starts_with(&[b'%', b'P', b'D', b'F']) {
        "PDF Document"
    } else if buffer.starts_with(&[b'P', b'K', 3, 4]) {
        "ZIP Archive (or DOCX/XLSX/JAR)"
    } else if buffer.starts_with(&[b'R', b'I', b'F', b'F']) {
        "RIFF container (WAV audio / AVI video)"
    } else {
        "Unknown Format / Plain Text"
    };

    println!("Format: {}", text.truecolor(57, 255, 20));
}

pub fn byte_entropy(data: &[u8]) {
    if data.is_empty() {
        println!("{}", "0.0".truecolor(191, 95, 255))
    }

    let mut counts = [0usize; 256];
    for &byte in data {
        counts[byte as usize] += 1;
    }

    let len = data.len() as f64;
    let entropy_value: f64 = -counts
        .iter()
        .filter(|&&count| count > 0)
        .map(|&count| {
            let probability = count as f64 / len;
            probability * probability.log2()
        })
        .sum::<f64>();
    let entropy = format!("{:.2}", entropy_value);
    println!("Entropy: {}", entropy.truecolor(191, 95, 255));
}

pub fn analyze_symbols(buffer: &[u8]) {
    let object = Object::parse(buffer).unwrap();

    let mut libraries = Vec::new();
    let mut imports = Vec::new();
    let mut exports = Vec::new();

    match object {
        Object::Elf(elf) => {
            // 2. Convert borrowed &str slices into owned .to_string() strings
            for lib in &elf.libraries {
                libraries.push((*lib).to_string());
            }

            for sym in &elf.dynsyms {
                if let Some(name) = elf.dynstrtab.get_at(sym.st_name) {
                    if !name.is_empty() {
                        if sym.is_import() {
                            imports.push(name.to_string());
                        } else {
                            exports.push(name.to_string());
                        }
                    }
                }
            }
        }
        _ => {}
    }
    println!(
        "Libraries: {}",
        format!("{:?}", libraries).truecolor(255, 215, 0)
    );
    println!(
        "Imports: {}",
        format!("{:?}", imports).truecolor(255, 215, 0)
    );
    println!(
        "Exports: {}",
        format!("{:?}", exports).truecolor(255, 215, 0)
    );
}
