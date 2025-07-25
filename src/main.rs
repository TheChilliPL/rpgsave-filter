use std::fs::remove_file;
use std::io;
use std::io::{Read, Write};
use std::process::{Command as StdCommand};
use clap::{Parser, Subcommand};
use eyre::{Context, ContextCompat, OptionExt};

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
    /// Installs the filter to the current git repository.
    ///
    /// It will use the path to rpgsave-filter that was used to call this command.
    Install,
}

fn main() -> eyre::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Smudge => {
            let mut input = String::with_capacity(10*1024);
            io::stdin().read_to_string(&mut input)?;

            // Input is formatted JSON
            let data: serde_json::Value = serde_json::from_str(&input)?;

            let minified_json = serde_json::to_string(&data)?;
            let compressed_base64 = lz_str::compress_to_base64(&minified_json);

            // Output is base64
            io::stdout().write_all(compressed_base64.as_bytes())?;
        },
        Command::Clean => {
            let mut input = String::with_capacity(10*1024);
            io::stdin().read_to_string(&mut input)?;

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
        Command::Install => {
            let this_path = std::env::current_exe().wrap_err("Failed to get current executable path")?;
            let this_path_str = this_path.to_str().wrap_err("Failed to convert current executable path to string")?;
            let this_path_str = this_path_str.replace("\\", "/");

            let cmd_status = StdCommand::new("git")
                .arg("status")
                .arg("--porcelain=1")
                .output().wrap_err("Failed to get git status")?;

            if !cmd_status.status.success() {
                return Err(eyre::eyre!("Failed to get git status"));
            }

            if !cmd_status.stdout.is_empty() {
                return Err(eyre::eyre!("Git repository is not clean. Please commit or stash changes before installing the filter."));
            }

            println!("Installing filter to current git repository");
            let cmd_clean = StdCommand::new("git")
                .arg("config")
                .arg("--local")
                .arg("filter.rpgsave.clean")
                .arg(format!("{} clean", this_path_str))
                .status().wrap_err("Failed to set clean filter")?;

            if !cmd_clean.success() {
                return Err(eyre::eyre!("Failed to set clean filter"));
            }

            let cmd_smudge = StdCommand::new("git")
                .arg("config")
                .arg("--local")
                .arg("filter.rpgsave.smudge")
                .arg(format!("{} smudge", this_path_str))
                .status().wrap_err("Failed to set smudge filter")?;

            if !cmd_smudge.success() {
                return Err(eyre::eyre!("Failed to set smudge filter"));
            }

            let cmd_required = StdCommand::new("git")
                .arg("config")
                .arg("--local")
                .arg("filter.rpgsave.required")
                .arg("true")
                .status().wrap_err("Failed to set filter as required")?;

            if !cmd_required.success() {
                return Err(eyre::eyre!("Failed to set filter as required"));
            }

            println!("Checking out repository");
            remove_file(".git/index")?;

            let cmd_checkout = StdCommand::new("git")
                .arg("checkout")
                .arg("HEAD")
                .arg("--")
                .arg(".")
                .status().wrap_err("Failed to checkout repository")?;

            if !cmd_checkout.success() {
                return Err(eyre::eyre!("Failed to checkout repository"));
            }

            println!("Successfully installed filter");
        },
    }

    Ok(())
}
