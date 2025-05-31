## XPManager:
It's good to have a CLI tool that manages your passwords and lets you control them and quickly create new passwords of various sizes to suit your needs. This is where XPManager comes in to help you manage passwords, and also allows you to work with files/folders and secure them with the **Fernet** encryption.

## v2.3.0, What's New?
- Fix: 
    - Decoded string display.
- New: 
    - Encryption and decryption directory with threads (default behavior), see [encryption manager](https://xpmanager.github.io/docs/usage/encryption-manager).
    - Encryption and decryption directory without threads (`--no-threads`), 
    see [encrypt directory](https://xpmanager.github.io/docs/usage/encryption-manager#encrypt-directory) and [decrypt directory](https://xpmanager.github.io/docs/usage/encryption-manager#decrypt-directory).

## Documentation:
See [XPManager documentation](https://xpmanager.github.io/docs/intro)

## Installation Instructions:
See [XPManager installation instructions](https://xpmanager.github.io/docs/installation)

## Install with Crates.io:
```sh
$ cargo install XPManager
$ xpm --version
```

## Cargo:
- Clone the repo:
```sh
$ git clone https://github.com/xpmanager/XPManager.git
$ cd XPManager
```
- Run
```sh
$ cargo run -- --version
```
- Test
```sh
$ cargo test
```
- Build with Release
```sh
$ cargo build --release
```
- Build deb package
```sh
$ cargo install cargo-deb
$ cargo deb
```

## Usage:
See [XPManager usage guide](https://xpmanager.github.io/docs/usage)

## Exit Codes:
See [XPManager exit codes](https://xpmanager.github.io/docs/errors)

---
> By [Mohaned Sherhan (Mr.x)](https://github.com/Mohaned2023)