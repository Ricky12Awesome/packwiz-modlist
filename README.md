[![crates.io](https://img.shields.io/crates/v/packwiz-modlist.svg)](https://crates.io/crates/packwiz-modlist)
[![docs.rs](https://docs.rs/packwiz-modlist/badge.svg)](https://crates.io/crates/packwiz-modlist)
[![license](https://img.shields.io/github/license/Ricky12Awesome/packwiz-modlist)](https://github.com/Ricky12Awesome/packwiz-modlist/blob/main/LICENSE)

# Packwiz ModList

Creates a modlist from [packwiz](https://packwiz.infra.link/)

## Install

```shell
cargo install packwiz-modlist
```

## Usage

if you run `packwizml` without any arguments it will try to 
create a `modlist.md` in the directory you executed the command in

### Options
```shell
# Displays help
packwizml --help # short: -h

# Specify a project path
#
# other path options are relative to this by default
# to disable this, add '-O' for output and '-M' for mods
packwizml --path ./ # short: -p

# Specify a mods directory
#
# to disable being relative to '--path' add '-O'
packwizml --output modlist.md # short: -o

# Specify a mods directory
#
# to disable being relative to '--path' add '-M'
packwizml --mods ./mods # short: -m

# Overwrite output file if it exists
packwizml --force # short: -F

# Priority: trace > debug > silent
# Shows up to trace info (more then debug)
packwizml --trace # short: -t

# Shows up to debug info
packwizml --debug # short: -d

# Silents logs info
packwizml --silent # short: -s
```

### Format

Specify a custom format
default: `[{NAME}]({URL}) - {DESCRIPTION}\n`

```shell
packwizml --format "[{NAME}]({URL}) - {DESCRIPTION}\n" # short: -f
```

#### Placeholders

| Placeholder                 | Description                         |
|:----------------------------|:------------------------------------|
| `{NAME}`,`{TITLE}`          | Gets the name of the project        |
| `{DESCRIPTION}`,`{SUMMARY}` | Gets the description of the project |
| `{URL}`                     | Gets the URL of the project         |
| `{SLUG}`                    | Gets the slug of the project        |

## Todo
- Support other formats
- Cache
- More placeholders
- CI/CD Integration