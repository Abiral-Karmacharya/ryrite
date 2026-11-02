mod file;
mod static_check;

use crate::file::read_file;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if let Err(e) = read_file(args[1].clone(), |chunk| {
        static_check::convert_to_string(chunk)
    }) {
        println!("{e}")
    }
    if let Err(e) = read_file(args[1].clone(), |chunk| {
        static_check::detect_file_format(chunk)
    }) {
        println!("{e}")
    }
    if let Err(e) = read_file(args[1].clone(), |chunk| static_check::byte_entropy(chunk)) {
        println!("{e}")
    }
    if let Err(e) = read_file(args[1].clone(), |chunk| {
        static_check::analyze_symbols(chunk)
    }) {
        println!("{e}")
    }
}
