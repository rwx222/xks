# xks - Git Profile Switcher with SSH Key Management

`xks` is a simple CLI tool for switching between multiple Git profiles,
seamlessly managing both `.gitconfig` and SSH keys. It allows you to save,
apply, and remove profiles while ensuring that only the necessary files are
modified.

## Features
- Save and switch between multiple Git profiles.
- Automatically updates `.gitconfig` and SSH keys.
- Ensures that only default Git and SSH files are managed.
- Lightweight, with just one external Rust dependency (`sha2`).

## Installation

### Install a Pre-Compiled Binary

Visit [https://github.com/andresdotsh/xks/releases](https://github.com/andresdotsh/xks/releases)
and download the appropriate archive for your operating system and architecture.

Extract the `xks` binary from the archive into a directory in your `$PATH`,
such as `/usr/local/bin/`.

### Build from Source

You can also build `xks` from source using Rust:

```sh
cargo build --release
cp target/release/xks /usr/local/bin/
```

Or move it to any directory included in your `$PATH`.

## How It Works

`xks` manages only the default Git and SSH configuration files (`current_files`):

```
~/.gitconfig
~/.ssh/config
~/.ssh/id_ed25519  (also id_ed25519.pub)
~/.ssh/id_ecdsa    (also id_ecdsa.pub)
~/.ssh/id_rsa      (also id_rsa.pub)
~/.ssh/id_dsa      (also id_dsa.pub)
```

All data is stored in `~/.xks`. Custom SSH keys or additional Git configuration
files are ignored.

## Usage

```sh
xks <command> [options]
```

### Commands

- `xks save <profile>` – Save the **current_files** as a profile.
- `xks use <profile>` – Apply a saved profile.
- `xks use -` – Switch back to the previously used profile.
- `xks remove <profile>` – Delete a saved profile.
- `xks discard` – Remove the **current_files**.
- `xks version` – Show the version number.
- `xks help` – Show usage information.

### Options

- `-y` – Skip confirmation prompts.

## Examples

```sh
xks                # List saved profiles and current_files state
xks save work      # Save current_files as 'work' profile
xks use personal   # Switch to 'personal' profile
xks use -          # Switch back to the previous profile
xks remove alex    # Delete 'alex' profile
xks discard        # Remove current_files
```

## License

`xks` is released under the MIT License.
