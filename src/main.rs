use std::{io::Write, process::Command};

use anyhow::Ok;
use clap::Parser;
use cli::Cli;
use k8s::get_secret_values;
use serde_json::json;
use tempfile::NamedTempFile;

mod cli;
mod k8s;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli_args = Cli::parse();

    let secret_values = get_secret_values(cli_args.selector, cli_args.namespace).await?;
    let json_string = json!(secret_values).to_string();
    let json_bytes = json_string.as_bytes();

    let mut tmpfile = NamedTempFile::new()?;
    tmpfile.write_all(json_bytes)?;
    let tmpfile_path = tmpfile.path().to_str().unwrap();

    let command = format!(
        "nix {} {} --override-input {} file+file://{} --refresh",
        cli_args.command, cli_args.flake, cli_args.input, tmpfile_path
    );

    let mut process = Command::new("/bin/sh")
        .arg("-c")
        .arg(command)
        .spawn()
        .expect("Nix command failed");

    let status = process.wait().expect("Failed to wait for command");

    if !status.success() {
        eprintln!("Command failed with exit code: {}", status);
        std::process::exit(1);
    }

    Ok(())
}
