# labels

`labels` is a CLI utility to synchornize your labels in a specific GitHub repo.

## Config file

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
