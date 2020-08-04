use crate::common;
use flate2::write::GzDecoder;
use flate2::{Compression, GzBuilder};
use std::io::Write;
use tar::{Archive, Builder};

pub fn pack(files: &Vec<common::File>) -> Result<Vec<u8>, String> {
    let mut ar = Builder::new(Vec::new());

    let compressed_data = Vec::new();
    let mut gz = GzBuilder::new().write(compressed_data, Compression::fast());

    let data = common::validate_files(&files)?;
    println!("[#] preparing files");
    for f in data.iter() {
        println!("[*] adding: {}", f);
        ar.append_path(f)
            .or(Err(String::from("Failed to read file")))?;
    }

    let data = ar
        .into_inner()
        .or(Err(String::from("Failed reading file archive")))?;
    println!("[!] Tar size: {}\n[*] compressing...", data.len());
    gz.write_all(&data)
        .or(Err(String::from("Failed to compress")))?;
    let compressed = gz.finish().or(Err(String::from("Failed to compress")))?;
    println!("[!] Gziped size: {}", compressed.len());
    Ok(compressed)
}

pub fn unpack(compressed: &[u8]) -> Result<(), String> {
    println!(
        "[!] Gzipped size: {}\n[*] decompressing...",
        compressed.len()
    );
    let mut writer = Vec::new();
    let mut decoder = GzDecoder::new(writer);
    decoder
        .write_all(compressed)
        .or(Err(String::from("Failed to decompress stream")))?;
    writer = decoder
        .finish()
        .or(Err(String::from("Issue with compression")))?;

    println!("[!] Tarbal size: {}\n[*] unpacking...", writer.len());
    let mut ar = Archive::new(&writer[..]);
    ar.unpack(".")
        .or(Err(String::from("Failed to unpack tarbal")))?;

    Ok(())
}
