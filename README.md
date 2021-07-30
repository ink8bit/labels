# labels

[![Rust](https://github.com/ink8bit/labels/actions/workflows/rust.yml/badge.svg)](https://github.com/ink8bit/labels/actions/workflows/rust.yml)

`labels` is a CLI utility to synchornize your labels in a specific GitHub repo.

## Setup

### Config file

You should create a configuration file called `.labelsrc` in the root of your project.
It should be a valid JSON with the following structure:

```json
{
  "owner": "repo_owner",
  "repo": "repo_name",
  "labels": [
    {
      "name": "bug",
      "description": "Something isn't working",
      "color": "d73a4a"
    },
    {
      "name": "enhancement",
      "description": "New feature or request",
      "color": "a2eeef"
    }
  ]
}
```

#### Note:
Emoji can be added to label names, using either native emoji or colon-style markup. For example, typing `:beetle:` will render the emoji :beetle:. For a full list of available emoji and codes, see "[Emoji cheat sheet.](https://github.com/ikatyang/emoji-cheat-sheet)".
```json
{
  "name": ":beetle: bug",
  "description": "Something isn't working",
  "color": "d73a4a"
}
```


### Token

You should create a [personal access token](https://docs.github.com/en/github/authenticating-to-github/keeping-your-account-and-data-secure/creating-a-personal-access-token) using GitHub UI with scope values:

- `repo`
- `read:user`

And export your token as an environment variable in your `.zshrc` (or `.bashrc`):

```shell
export LABELS_TOKEN="GITHUB_TOKEN_VALUE"
```

## Usage

```console
USAGE:
    labels [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help      Prints this message or the help of the given subcommand(s)
    list      Prints labels in current repository (first 100 items)
    update    Updates all labels in current repository
```

## Subcommands

Print the first 100 labels from the current repo:

```console
labels list
```

Update all labels from the current repo:

```console
labels update
```
