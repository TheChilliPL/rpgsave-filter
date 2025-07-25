use std::io;
use std::io::{Read, Write};
use clap::{Parser, Subcommand};
use eyre::{Context, OptionExt};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Used when checking out the file. Minifies the JSON and compresses with lzstring to base 64.
    Smudge,
    /// Used when staging the file. Decodes the save data to readable and formatted JSON.
    Clean,
}

fn main() -> eyre::Result<()> {
    let cli = Cli::parse();

    let mut input = String::with_capacity(10*1024);
    io::stdin().read_to_string(&mut input)?;

    match cli.command {
        Command::Smudge => {
            // Input is formatted JSON
            let data: serde_json::Value = serde_json::from_str(&input)?;

            let minified_json = serde_json::to_string(&data)?;
            let compressed_base64 = lz_str::compress_to_base64(&minified_json);

            // Output is base64
            io::stdout().write_all(compressed_base64.as_bytes())?;
        },
        Command::Clean => {
            // Input is base64
            let minified_json_utf16 = lz_str::decompress_from_base64(&input)
                .ok_or_eyre("Failed to decompress base64")?;
            let mut minified_json_iter = char::decode_utf16(minified_json_utf16);
            let minified_json = minified_json_iter.try_fold(
                String::with_capacity(minified_json_iter.size_hint().0),
                |mut acc, result| {
                    result.map(|value| {
                        acc.push(value);
                        acc
                    })
                }
            ).with_context(|| "Failed to decode UTF-16")?;

            let data: serde_json::Value = serde_json::from_str(&minified_json)?;

            let formatted_json = serde_json::to_string_pretty(&data)?;

            // Output is formatted JSON
            io::stdout().write_all(formatted_json.as_bytes())?;
        },
    }

    Ok(())
}
