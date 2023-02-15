pub mod chunk;
pub mod chunk_type;
pub mod png;

use std::path::Path;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

/// Encodes a message into a PNG file and saves the result
pub fn encode<P: AsRef<Path>>(
    file_path: P,
    chunk_type: &str,
    message: &str,
    output_file: Option<P>,
)-> Result<()> {
    todo!()
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode<P: AsRef<Path>>(file_path: P, chunt_type: &str) -> Result<()> {
    todo!()
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove<P: AsRef<Path>>(file_path: P, chunt_type: &str) -> Result<()> {
    todo!()
}

/// Prints all of the chunks in a PNG file
pub fn print_chunks<P: AsRef<Path>>(file_path: P) -> Result<()> {
    todo!()
}