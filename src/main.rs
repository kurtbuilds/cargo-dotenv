/// cargo dotenv
/// This package is basically a workaround for the lack of support for EnvFile in IntelliJ-Rust
/// Using this package, you can configure your cargo
use std::{env, fs, process};
use ::env::EnvFile;
use clap::Parser;

/// Example:
/// cargo dotenv check
/// cargo dotenv -e .env.production -e .env.staging -- run --host 0.0.0.0 --port 5000
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    /// Specify the envfile
    #[clap(short, long)]
    pub env_files: Vec<String>,

    /// Use this if you want to skip any files that are missing in env_file.
    /// Could be useful for global IntelliJ configuration, when you have different
    /// .env file paths for different projects (e.g. workspace vs not).
    #[clap(long)]
    pub skip_missing: bool,

    pub command: Vec<String>,
}


fn main() {
    let args = env::args_os().skip(1);
    let cli = Cli::parse_from(args);
    let mut env_files = cli.env_files;
    if env_files.is_empty() {
        if fs::metadata(".env").is_ok() {
            env_files.push(".env".to_string());
        } else if fs::metadata("../.env").is_ok() {
            env_files.push("../.env".to_string());
        }
    } else {
        if cli.skip_missing {
            env_files.retain(|path| fs::metadata(path).is_ok());
        }
    }
    for path in env_files {
        if fs::metadata(&path).is_err() {
            if cli.skip_missing {
                continue;
            }
            panic!("{} does not exist.", path);
        }
        let envfile = EnvFile::read(path);
        for (key, value) in envfile.iter() {
            env::set_var(key, value);
        }
    }
    let mut command = cli.command.into_iter().peekable();
    while let Some(p) = command.peek() {
        if !p.contains('=') {
            break;
        }
        let pair = p.splitn(2, '=').collect::<Vec<_>>();
        env::set_var(pair[0], pair[1]);
        command.next();
    }
    // let executable = command.next().expect("Must provide a command");
    let code = process::Command::new("cargo")
        .args(command)
        .status()
        .expect("Failed to execute command")
        .code()
        .unwrap();
    process::exit(code);
}
