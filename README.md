# Packwiz ModList

Creates a modlist from [packwiz](https://packwiz.infra.link/)

## Install

```shell
cargp install packwiz-modlist
```

## Usage

Run this inside the packwiz project directory, and it will create a `modlist.md` in that directory

```shell
packwizml 
```

Run this for a full list of arguments

```shell
packwizml --help
packwizml -h # Short version
```

If you want to specify a packwiz project directory you can do

```shell
packwizml --path ./
packwizml -p ./ # Short version
```

If you want a custom output path

```shell
packwizml --output modlist.md
packwizml -o modlist.md # Short version
```

If you want a custom format

```shell
packwizml --format "[{NAME}]({URL}) - {DESCRIPTION}\n"
packwizml -f "[{NAME}]({URL}) - {DESCRIPTION}\n" # Short version
```

## Placeholders

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
