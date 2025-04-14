# gus

Git User Switcher - Simple command line utility for quickly/easily switching between local git users

[![ci](https://github.com/jamieweavis/gus/actions/workflows/ci.yml/badge.svg)](https://github.com/jamieweavis/gus/actions)
[![downloads](https://img.shields.io/github/downloads/jamieweavis/gus/total)](https://github.com/jamieweavis/gus/releases)
[![version](https://img.shields.io/github/v/release/jamieweavis/gus)](https://github.com/jamieweavis/gus/releases)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/jamieweavis/gus/blob/main/LICENSE)

## Installation

Install via [Homebrew](https://brew.sh) (currently macOS only):

```sh
brew install jamieweavis/tap/gus
```

## Usage

### Switch

Switch to a user by their ID:

```sh
$ gus 0
Switched git user to: Johnny <john.doe@gmail.com>
```

Switch to the previous user:

```sh
$ gus -
Switched git user to: John Doe <john.doe@corporation.io>
```

### List

List users in your `gus` config:

```sh
$ gus ls
* 0: Johnny <john.doe@gmail.com>
  1: John Doe <john.doe@corporation.io>
```

### Edit

Edit your `gus` config in your configured `$EDITOR`:

```sh
$ gus edit
```

> [!NOTE]
> Alternatively you can edit the file at `~/.config/gus.toml` - see the [configuration](#configuration) section for more details

### Help

Print usage and package information:

```sh
$ gus

gus 1.0.1 (https://github.com/jamieweavis/gus)

Git User Switcher - Simple command line utility for quickly/easily switching between local git users

Usage: gus <command>

COMMANDS
  <id>       Switch to user with the provided ID
  -          Switch to the previous user
  list, ls   List users and their IDs
  config     Open `~/.config/gus.toml` in your $EDITOR
```

## Configuration

When you run `gus` for the first time a `~/.config/gus.toml` file is created. This file is used to store your git users and is prepopulated with a your current git user.

As this is a Rust project, the config file is in [TOML](https://toml.io/en/) format and is structured as follows:

### Initial Config

```toml
previous_user = 0
current_user = 0

[[users]]
name = "<your current git user name>"
email = "<your current git user email>"
```

Additional users can be added to the users array by using [Array of Tables](https://toml.io/en/v1.0.0#array-of-tables) syntax:

### Example Config

```toml
previous_user = 1
current_user = 0

[[users]]
name = "Johnny"
email = "john.doe@gmail.com"

[[users]]
name = "John Doe"
email = "john.doe@company.com"
```
> [!WARNING]
> The `previous_user` and `current_user` fields are updated by gus when you switch users - don't manually edit these fields!

## Building Locally

### Production

Compile a release binary:

```sh
cargo build --release
```

Run the release binary:

```sh
cargo run --release
```

### Development

Run in development mode:

```sh
cargo run
```

### Install

Install the binary to your system:

```sh
cargo install --path .
```

Uninstall the binary from your system:

```sh
cargo uninstall gus
```

## Disclaimer

I'm not a Rust developer, this is just for fun!

## Built With

- [Rust](https://github.com/rust-lang/rust)

## Related

- [jamieweavis/homebrew-tap](https://github.com/jamieweavis/homebrew-tap) - Homebrew tap for my brew formulae

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
