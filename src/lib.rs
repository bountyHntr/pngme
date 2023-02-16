pub mod chunk;
pub mod chunk_type;
pub mod png;

use std::{path::Path, str::FromStr};
use png::Png;
use chunk::Chunk;
use chunk_type::ChunkType;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

/// Encodes a message into a PNG file and saves the result
pub fn encode<P: AsRef<Path>>(
    file_path: P,
    chunk_type: &str,
    message: String,
    output_file: Option<P>,
)-> Result<()> {
    let mut png = Png::from_file(&file_path)?;

    let chunk_type = ChunkType::from_str(chunk_type)?;
    let chunk = Chunk::new(chunk_type, message.into());

    png.append_chunk(chunk);

    match output_file {
        Some(output_file) => png.to_file(output_file),
        None => png.to_file(file_path),
    }
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode<P: AsRef<Path>>(file_path: P, chunt_type: &str) -> Result<()> {
    let png = Png::from_file(&file_path)?;
    let chunk = png.chunk_by_type(chunt_type).ok_or("chunk not found")?;
    println!("{}", chunk.data_as_string()?);
    Ok(())
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove<P: AsRef<Path>>(file_path: P, chunk_type: &str) -> Result<()> {
    let mut png = Png::from_file(&file_path)?;
    png.remove_chunk(chunk_type)?;
    png.to_file(file_path)
}

/// Prints all of the chunks in a PNG file
pub fn print_chunks<P: AsRef<Path>>(file_path: P) -> Result<()> {
    println!("{}", Png::from_file(&file_path)?);
    Ok(())
}