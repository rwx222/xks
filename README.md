# xks - Git Profile Switcher with SSH Key Management

`xks` is a CLI tool for switching between multiple Git profiles,
seamlessly managing both `.gitconfig` and `SSH keys`. It allows you to save,
apply, and remove profiles while ensuring that only the necessary files are
modified.

## Features

- Save and switch between multiple Git profiles.
- Automatically updates `.gitconfig` and SSH keys.
- Ensures that only default Git and SSH files are managed.
- Lightweight, with just one external Rust dependency (`sha2`).

## Installation

You can install `xks` in three different ways:

### (Option 1) Install with curl or wget `(recommended)`

To install automatically, run one of the following commands:

With **curl**:

```sh
curl -fsSL https://xks.rwx222.com/install.sh | sh
```

Or with **wget**:

```sh
wget -qO- https://xks.rwx222.com/install.sh | sh
```

---

### (Option 2) Download a Pre-Compiled Binary

Visit [GitHub Releases](https://github.com/rwx222/xks/releases) and download
the appropriate archive for your **OS and architecture**.

Then, extract the `xks` binary and move it to a directory in your `$PATH`,
such as:

```sh
tar -xzf xks-<version>-<os>-<arch>.tar.gz -C /usr/local/bin/
```

---

### (Option 3) Build from Source `(requires rust)`

If you prefer to build `xks` yourself, you can compile it with Rust:

```sh
cargo build --release
cp target/release/xks /usr/local/bin/
```

Or move it to any directory included in your `$PATH`.

---

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

- `xks` List saved profiles and **current_files** state.
- `xks save <profile>` Save the **current_files** as a profile.
- `xks use <profile>` Apply a saved profile.
- `xks use -` Switch back to the previously used profile.
- `xks remove <profile>` Delete a saved profile.
- `xks discard` Remove the **current_files**.
- `xks version` Show the version number.
- `xks help` Show usage information.

### Options

- `-y` Skip confirmation prompts.

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

---

# Why I Built `xks`

I created `xks` because I often need to switch between different Git profiles
for work and personal projects.

Since SSH keys cannot be shared between multiple accounts, managing them
manually quickly became frustrating. While there are existing tools and other
techniques to solve this problem, I wanted to build my own as a way to learn
**Rust**.

Although `xks` was originally designed for Git profiles, it can be useful in
different scenarios:

- Switching between Git profiles, with or without SSH keys.
- Managing SSH keys independently, even without Git, for accessing multiple
  remote servers.
- Handling both Git profiles and SSH keys together, making it easy to manage
  multiple identities.

This allows you to:

- Switch between SSH keys for different remote servers.
- Separate work and personal Git profiles.
- Manage multiple Git accounts for freelance clients.
- Use different Git configurations per profile.

If you find yourself in a similar situation, give `xks` a try! 🚀
