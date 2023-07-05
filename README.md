This is a tiny utility that injects env variables before running cargo commands.

I built this utility specifically because IntelliJ Rust does not support
`.env` files, nor does the `EnvFile` plugin support Rust.

Otherwise, I recommend adding env variables through `just`, your shell, or other means.

# Usage

```bash
# Runs `cargo run` with env coming from `.env`
cargo dotenv run
```

```bash
# Runs `cargo check` with env coming from `.env.production`
cargo dotenv -e .env.production check
```

You can run dotenv recursively arbitrarily. This fact is useless, but I find it amusing.

```bash
cargo dotenv dotenv dotenv dotenv dotenv dotenv dotenv dotenv dotenv check
```

# Installation

```bash
cargo install cargo-dotenv
```

# Issue Tracker

- EnvFile: https://github.com/ashald/EnvFile/issues/71
- IntelliJ (YouTrack): https://youtrack.jetbrains.com/issue/IDEA-137820