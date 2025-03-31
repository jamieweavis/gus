# gus

> 🐻 Git user switcher for quickly switching between local git users

[![build](https://github.com/jamieweavis/gus/actions/workflows/build.yml/badge.svg?branch=main)](https://github.com/jamieweavis/gus/actions)
[![version](https://img.shields.io/github/v/release/jamieweavis/gus)](https://github.com/jamieweavis/gus/releases)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/jamieweavis/gus/blob/main/LICENSE)

## Installation

Coming soon!

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

When you run `gus` for the first time a `.config/gus.toml` file is created in your home directory. This file is used by gus to store your git users and is prepopulated with a your current git user.

As this is a Rust project, the config file is in [TOML](https://toml.io/en/) format. The config file is structured as follows:

### Initial config file

```toml
previous_user = 0
current_user = 0

[[users]]
name = "<your current git user name>"
email = "<your current git user email>"
```

Additional users can be added to the users array using by [Array of Tables](https://toml.io/en/v1.0.0#array-of-tables) syntax:

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

_The `previous_user` and `current_user` fields are updated by gus when you switch users - you should not edit these fields._

## Building Locally

### Production

Build a release version of the application

```sh
cargo build --release
```

Run the application in release mode

```sh
cargo run --release
```

### Development

Run the application in development mode

```sh
cargo run
```

### Install

Install the binary to your local cargo bin

```sh
cargo install --path .
```

Uninstall the binary from your local cargo bin

```sh
cargo uninstall gus
```

## Built With

- [Rust](https://github.com/rust-lang/rust)

## Disclaimer

I'm not a Rust engineer, this is just for fun! 😄

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
