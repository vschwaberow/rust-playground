use clap::{Parser, Subcommand};
use std::fs;
use std::io::{self, Read, Write};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Pack { input: String, output: String },
    Depack { input: String, output: String },
}

fn run_length_encode(input: &[u8]) -> Vec<(u8, usize)> {
    let mut encoded = Vec::new();
    let mut iter = input.iter().peekable();
    while let Some(&byte) = iter.next() {
        let mut count = 1;
        while let Some(&&next) = iter.peek() {
            if next == byte {
                count += 1;
                iter.next();
            } else {
                break;
            }
        }
        encoded.push((byte, count));
    }
    encoded
}

fn run_length_decode(encoded: &[(u8, usize)]) -> Vec<u8> {
    let mut decoded = Vec::new();
    for &(byte, count) in encoded {
        for _ in 0..count {
            decoded.push(byte);
        }
    }
    decoded
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Pack { input, output } => {
            let input_data = fs::read(input)?;
            let input_size = input_data.len();
            let encoded_data = run_length_encode(&input_data);
            let encoded_size = encoded_data.len();
            let mut output_file = fs::File::create(output)?;
            for &(byte, count) in &encoded_data {
                output_file.write_all(&[byte])?;
                output_file.write_all(&(count as u32).to_le_bytes())?;
            }
            println!("Input file size: {} bytes", input_size);
            println!("Encoded file size: {} bytes", encoded_size * 5);
            println!(
                "Compression ratio: {:.2}%",
                100.0 * (encoded_size * 5) as f64 / input_size as f64
            );
        }
        Commands::Depack { input, output } => {
            let mut input_file = fs::File::open(input)?;
            let mut encoded_data = Vec::new();
            let mut buffer = [0u8; 5];
            while input_file.read_exact(&mut buffer).is_ok() {
                let byte = buffer[0];
                let count =
                    u32::from_le_bytes([buffer[1], buffer[2], buffer[3], buffer[4]]) as usize;
                encoded_data.push((byte, count));
            }
            let decoded_data = run_length_decode(&encoded_data);
            fs::write(output, decoded_data)?;
        }
    }

    Ok(())
}
