![Aetheric Banner](https://github.com/aetheric-oss/.github/blob/main/assets/readme-banner.png)

# `lib-common` Library

![GitHub stable release (latest by date)](https://img.shields.io/github/v/release/aetheric-oss/lib-common?sort=semver&color=green)
![GitHub release (latest by date including pre-releases)](https://img.shields.io/github/v/release/aetheric-oss/lib-common?include_prereleases)
[![Coverage Status](https://coveralls.io/repos/github/aetheric-oss/lib-common/badge.svg?branch=develop)](https://coveralls.io/github/aetheric-oss/lib-common)
![Sanity Checks](https://github.com/aetheric-oss/lib-common/actions/workflows/sanity_checks.yml/badge.svg?branch=main)
![Rust Checks](https://github.com/aetheric-oss/lib-common/actions/workflows/rust_ci.yml/badge.svg?branch=main)
![Python PEP8](https://github.com/aetheric-oss/lib-common/actions/workflows/python_ci.yml/badge.svg?branch=main)
![Arrow DAO Discord](https://img.shields.io/discord/853833144037277726?style=plastic)

## :telescope: Overview

Common functions and data types across the Aetheric microservices.

Directory:
- `src/`: Source Code and Unit Tests
- `tests/`: Integration Tests
- `docs/`: Module Documentation

## Installation

Install Rust with [Rustup](https://www.rust-lang.org/tools/install).

```bash
# Adds custom pre-commit hooks to .git through cargo-husky dependency
# !! Required for developers !!
cargo test
```

## Make

### Build and test

To ensure consistent build and test outputs, Arrow provides a Docker image with all required software installed to build and test Rust projects.
Using the Makefile, you can easily test and build your code.

```bash
# Run tests
make test

# Run build
make build
```

### Formatting

The Arrow docker image has some formatting tools installed which can fix your code formatting for you.
Using the Makefile, you can easily run the formatters on your code.
Make sure to commit your code before running these commands, as they might not always result in a desired outcome.

```bash
# Format TOML files
make toml-tidy

# Format Rust files
make rust-tidy

# Format Python files
make python-tidy

# Format all at once
make tidy
```

### Spell check

Before being able to commit, cspell will be used as a spelling checker for all files, making sure no unintended spelling errors are found.
You can run cspell yourself by using the following make target:
```bash
make cspell-test
```

If all spelling errors are fixed, but cspell still finds words that are unknown, you can add these words to the local project words list by running the following command:
```bash
make cspell-add-words
```

### Other make targets

There are additional make targets available. You can find all possible targets by running make without a target or use `make help`

## :scroll: Documentation
The following documents are relevant to this library:
- [Concept of Operations](./docs/conops.md)

## :busts_in_silhouette: About Us

Learn more about us: [Aetheric website](https://www.aetheric.nl)

## LICENSE Notice

Please note that lib-common is under BUSL license until the Change Date, currently the earlier of two years from the release date. Exceptions to the license may be specified by Aetheric Governance via Additional Use Grants, which can, for example, allow lib-common to be deployed for certain production uses. Please reach out to Aetheric to request a vote for exceptions to the license, or to move up the Change Date.
