# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Test Commands

### Rust

- Build: `cd core && cargo build [--release]`
- Test: `cd core && cargo test [--package PACKAGE_NAME] [--test TEST_NAME]`
- Run specific test: `cd core && cargo test --package PACKAGE_NAME::test_name -- --exact`
- Lint: `cd core && cargo clippy`

### Solidity

- Build contracts: `yarn workspace l1-contracts hardhat compile`
- Test contracts: `yarn workspace l1-contracts hardhat test [TEST_FILE]`
- Lint Solidity: `yarn solhint "contracts/**/*.sol"`

## Formatting & Coding Style

### Rust

- Use 4 spaces for indentation
- Follow Rust's idiomatic error handling with `anyhow::Result` and `?` operator
- Use `#[derive(...)]` for common traits
- Document public functions with `///` comments
- Type names use PascalCase; function names use snake_case
- Organize imports alphabetically and group std imports separately

### Solidity

- Use 4 spaces for indentation
- Include SPDX license identifier and pragma statement
- Document functions with NatSpec comments (`@notice`, `@dev`, `@param`, etc.)
- Use explicit visibility for functions and variables
- Constants in UPPER_SNAKE_CASE
- Function names in camelCase
