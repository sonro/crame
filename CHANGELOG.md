# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

#### `new` subcommand

- Cli interface for starting a crame project.
- Error handling if project directory already exists.
- Create project directory

#### Cli

- Command line interface parsing with [clap](https://github.com/clap-rs/clap).
- Setup [tracing](https://github.com/tokio-rs/tracing) for application logging.

#### Error handling

- Setup [anyhow](https://github.com/dtolnay/anyhow) and [thiserrror](https://github.com/dtolnay/thiserror) for error handling.
- Coloured error output using [colored](https://github.com/mackwic/colored).
- Exit code handling from [exitcode](https://github.com/benwilber/exitcode).
