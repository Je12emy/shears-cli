# Shears CLI

Shears is a CLI which aims to automate cutting release branches for repositories hosted on Gitlab.

![Landing Image](./assets/shears_landing.jpg)

## Requirements

You need to Gitlab private access token to interact with the Gitlab Rest API, you can read the [official documentation on how to generate one](https://docs.gitlab.com/ee/user/profile/personal_access_tokens.html). Furthermore, this token needs the `api` scope to create branch and a pull request on your repository.

## Usage

You can either use the shears CLI as a standalone CLI app or through a configuration file written in [TOML](https://toml.io/en/).

## As a Standalone CLI

Simply invoke the CLI and pass the following arguments.

| Name          | Description                                                                                                                                                                                     | Short Form | Long Form |
| ------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------- | --------- |
| Private Token | Personal access token used for authentication with Gitlab                                                                                                                                       | t          | token     |
| Project ID    | Your repository's ID, you may need write permissions on it. You may retrieve this ID on your [project's settings page](https://docs.gitlab.com/ee/user/project/settings/#view-project-settings) | i          | id        |
| Source Branch | The base branch used for creating the new release branch, when creating a PR this branch will be merged into the target branch                                                                  | s          | source    |
| Target Branch | The target branch for the new release pull request                                                                                                                                              | t          | target    |
| Gitlab URL    | The Gitlab URL, by default https://gitlab.com is used. If you are hosting your own Gitlab instance you will need to pass this argument                                                          | u          | url       |

Here's an example:

```bash
$ shears-cli -t glpat-dKnH2Q8A59cnucaDbDfR -i 46177309 -s main -d main

Please enter a branch name: release/1.1.2
New branch release/1.1.2 created!
URL: https://gitlab.com/Je12emy/test_project/-/tree/release/1.1.2
Please enter a PR title for branch "release/1.1.2": latest 1.1.2 release
New pull request for branch: "release/1.1.2" created!
URL: https://gitlab.com/Je12emy/test_project/-/merge_requests/13
```

Notice that you are asked for a branch name and a title for your PR.

## Using a  configuration file

If you don't want to use the same command multiple times, don't want to type that many arguments or even better, you wan to cut multiple release branches, then this is a great alternative. You will need a configuration file named: "config.toml" written in TOML, with the settings we discussed earlier. This configuration file should be placed in the following directories depending on your OS.

| OS      | Path                                                          |
| ------- | ------------------------------------------------------------- |
| Linux   | /home/user/.config/shears-cli                                 |
| Windows | C:\Users\user\AppData\Roaming\je12emy\shears-cli              |
| MacOs   | /Users/user/Library/Application Support/com.je12emy.shears-cli|

Here's a sample configuration file.

```TOML
private_token = "foo"
gitlab_url = "https://gitlab.com"

[[projects]]
project_id = "1111111"
base_branch = "develop"
target_branch = "main"

[[projects]]
project_id = "2222222"
base_branch = "develop"
target_branch = "main"
```

Here the `projects` key allows you to set-up many repositories, shears create a release branch and release PR for you on each project.

With this set-up you can simply call shears, and the configuration file will be read. Here's an example:

```bash
$ shears-cli
Please enter a branch name: release/1.2.3
New branch release/1.2.3 created!
URL: https://gitlab.com/Je12emy/test_project/-/tree/release/1.2.3
Please enter a PR title for branch "release/1.2.3": latest 1.2.3 release
New pull request for branch: "release/1.2.3" created!
URL: https://gitlab.com/Je12emy/test_project/-/merge_requests/14
```

# Installation

## Pre build Binaries

Check the [releases](https://github.com/Je12emy/shears-cli/releases) page and download a binary for your operating system. You can then place the binary on your path or anywhere else you prefer.

## Building from Source

This CLI is built using [rust](https://www.rust-lang.org/), so you can run it on build it using cargo.
