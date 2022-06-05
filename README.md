[![crates.io](https://img.shields.io/crates/v/packwiz-modlist.svg)](https://crates.io/crates/packwiz-modlist)
[![license](https://img.shields.io/github/license/Ricky12Awesome/packwiz-modlist)](https://github.com/Ricky12Awesome/packwiz-modlist/blob/main/LICENSE)

# Packwiz ModList

Creates a modlist from [packwiz](https://packwiz.infra.link/)

## Install

```shell
cargo install packwiz-modlist
```

## Usage

if you run `packwizml` without any arguments, it will print the output with the default format,
you can redirect the output using the `-o` flag or using `> filename`

### Options
```sh
# Displays help
packwizml --help # short: -h

# Prints about this program
packwizml --about

# Specify a project path
# other path options are relative to this by default
# to disable this, add '-O' for output and '-M' for mods
packwizml --path ./ # short: -p

# Specify a mods directory
# to disable being relative to '--path' add '-O'
packwizml --output modlist.md # short: -o

# Specify a mods directory
# to disable being relative to '--path' add '-M'
packwizml --mods ./mods # short: -m

# Overwrite output file if it exists
packwizml --force # short: -F

# Prints out all data as json so it can be used in scripts
packwizml --json

# Specify the lagging level
# possible values: Off, Error, Warn, Info, Debug, Trace
# default: Warn
packwizml --log-level=Off # short: -v

# Specify the color mode
# possible values: Auto, Always, Never
# default: Auto
packwizml --color-mode=Auto # short: -c

# Specify a custom format
# default: `- [{NAME}]({URL}) - {DESCRIPTION}\n`
packwizml --format "- [{NAME}]({URL}) - {DESCRIPTION}\n" # short: -f

# Specify how it should sort
# possible values: Name, Title, Slug, Id, None
packwizml --sort Name # short: -s
```

#### Placeholders

| Placeholder                  | Description                      |
|:-----------------------------|:---------------------------------|
| `{INDEX}`                    | Gets project index in the list   |
| `{NAME}`, `{TITLE}`          | Gets project name/title          |
| `{DESCRIPTION}`, `{SUMMARY}` | Gets project description/summary |
| `{URL}`                      | Gets project URL                 |
| `{SLUG}`                     | Gets project slug                |
| `{ID}`                       | Gets project id                  |

### Sorting

| Type                         | Description                      |
|:-----------------------------|:---------------------------------|
| `Name`, `Title`              | Sorts by project name            |
| `Slug`                       | Sorts by project slug            |
| `Id`                         | Sorts by project id              |
| `None`                       | Undetermined                     |

## Todo
* [ ] Sorting
* [ ] Automated Tests
* [ ] Packaging outside of cargo
* [ ] Packaging with Completions
* [ ] Caching (to avoid fetching project by url, if same version)
* [ ] Templates (kinda like preset-format)