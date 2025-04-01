# gus

> 🐻 Git user switcher for quickly switching between local git users

[![build](https://github.com/jamieweavis/gus/actions/workflows/build.yml/badge.svg?branch=main)](https://github.com/jamieweavis/gus/actions)
[![version](https://img.shields.io/github/v/release/jamieweavis/gus)](https://github.com/jamieweavis/gus/releases)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/jamieweavis/gus/blob/main/LICENSE)

## Installation

Install via [Homebrew](https://brew.sh):

```sh
brew tap jamieweavis/gus
brew install gus
```

## Usage

### Help

Print usage and package information:

```sh
$ gus

gus 1.0.0 (https://github.com/jamieweavis/gus)

🐻 Git user switcher for quickly switching between local git users

Usage: gus <command>

COMMANDS
  <id>       Switch to user with the provided ID
  -          Switch to the previous user
  list, ls   List users and their IDs
  config     Open `~/.config/gus.toml` in your $EDITOR
```

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

List users configured in your gus config file:

```sh
$ gus ls
* 0: Johnny <john.doe@gmail.com>
  1: John Doe <john.doe@corporation.io>
```

### Edit

Edit your gus config file in your configured shell `$EDITOR` (alternatively you can manually edit the file at `~/.config/gus.toml`, see the [configuration](#configuration) section for more details):

```sh
$ gus edit
```

## Configuration

When you run `gus` for the first time a `~/.config/gus.toml` file is created. This file is used by gus to store your git users and is prepopulated with a your current git user.

As this is a Rust project, the config file is in [TOML](https://toml.io/en/) format. The config file is structured as follows:

### Initial config file

```toml
previous_user = 0
current_user = 0

[[users]]
name = "<your current git user name>"
email = "<your current git user email>"
```

Additional users can be added to the users array by using [Array of Tables](https://toml.io/en/v1.0.0#array-of-tables) syntax:

### Example config file

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

_The `previous_user` and `current_user` fields are updated by gus when you switch users - you should not edit these fields manually._

## Building Locally

### Production

Compile a release binary of gus:

```sh
cargo build --release
```

Run the release binary:

```sh
cargo run --release
```

### Development

Run gus in development mode:

```sh
cargo run
```

### Install

Install the binary to your local crates:

```sh
cargo install --path .
```

Uninstall the binary from your local crates:

```sh
cargo uninstall gus
```

## Built With

- [Rust](https://github.com/rust-lang/rust)

## Disclaimer

I'm not a Rust engineer, this is just for fun!

## Related

- [homebrew-gus](https://github.com/jamieweavis/homebrew-gus) - 🍺 Brew tap & formula for gus

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
