use std::path::PathBuf;
use clap::{Parser, Subcommand};
use pngme::{self, Result};

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: Commands
}


#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// Encodes a message into a PNG file
    Encode {
        file_path: PathBuf,
        chunk_type: String,
        message: String,
        output_file: Option<PathBuf>,
    },
    /// Searches for a message hidden in a PNG file
    Decode {
        file_path: PathBuf,
        chunk_type: String,
    },
    /// Removes a chunk from a PNG file
    Remove {
        file_path: PathBuf,
        chunk_type: String,
    },
    /// Prints all of the chunks in a PNG file
    Print {
        file_path: PathBuf,
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Encode {
            file_path,
            chunk_type,
            message,
            output_file,
        } => pngme::encode(file_path, &chunk_type, &message, output_file)?,
        Commands::Decode {file_path, chunk_type} => pngme::decode(file_path, &chunk_type)?,
        Commands::Remove {file_path, chunk_type} => pngme::remove(file_path, &chunk_type)?,
        Commands::Print {file_path} => pngme::print_chunks(file_path)?,
    }

    Ok(())
}
