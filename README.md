# GitHub Action Document Writer

Provides an opinionated command line application to generated READMEs for
[GitHub Actions] and [GitHub Workflows].

## Usage

### Documenting GitHub Actions

```
$ github-action-doc action -h
Generate documentation for a Github action

USAGE:
    github-action-doc action <ACTION_FILE>

ARGS:
    <ACTION_FILE>

OPTIONS:
    -h, --help    Print help information

```

### Documenting GitHub Workflows

```
$ github-action-doc workflow -h
Generate documentation for a Github workflow

USAGE:
    github-action-doc workflow <WORKFLOW_FILE>

ARGS:
    <WORKFLOW_FILE>

OPTIONS:
    -h, --help    Print help information
```

## Examples

### Actions

* [`actions/checkout@v3`](https://gist.github.com/foo4u/3935d6d9b41fe92b19077515c7a035e5#file-checkout_readme-md)
* [`actions/setup-java@v3`](https://gist.github.com/foo4u/3935d6d9b41fe92b19077515c7a035e5#file-setup_java_jdk_readme-md)

[GitHub Actions]: https://docs.github.com/en/actions/learn-github-actions/essential-features-of-github-actions
[GitHub Workflows]: https://github.com/features/actions
