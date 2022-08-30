# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

#### `new` subcommand

- Cli interface for starting a crame project.
- Error handling if project directory already exists.
- Create project directory.
- Use a template to create project files.
- Check if within an existing git repository or initialize it with [git2]
- Create config in 'Crame.toml' file using [serde] and [toml].
- Create justfile as build system.

#### Cli

- Command line interface parsing with [clap].
- Setup [tracing] for application logging.

#### Error handling

- Setup [anyhow] and [thiserrror] for error handling.
- Coloured error output using [colored].
- Exit code handling from [exitcode].

[git2]: https://github.com/rust-lang/git2-rs
[clap]: https://github.com/clap-rs/clap
[tracing]: https://github.com/tokio-rs/tracing
[anyhow]: https://github.com/dtolnay/anyhow
[thiserrror]: https://github.com/dtolnay/thiserror
[colored]: https://github.com/mackwic/colored
[exitcode]: https://github.com/benwilber/exitcode
[serde]: https://github.com/serde-rs/serde
[toml]: https://github.com/alexcrichton/toml-rs
