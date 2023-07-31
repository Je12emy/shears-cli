# Shears CLI

Shears is a CLI which aims to automate cutting release branches for repositories hosted on Gitlab.

![Landing Image](./assets/shears_landing.jpg)

## Requirements

You need to Gitlab private access token to interact with the Gitlab Rest API, you can read the [official documentation on how to generate one](https://docs.gitlab.com/ee/user/profile/personal_access_tokens.html). Furthermore, this token needs the `api` scope to create branch and a pull request on your repository.

## Usage

As a CLI application you can check out the help documentation with the `-h` flag.

## Pre build Binaries

Check the [releases](https://github.com/Je12emy/shears-cli/releases) page and download a binary for your operating system. You can then place the binary on your path or anywhere else you prefer.

## Building from Source

This CLI is built using [rust](https://www.rust-lang.org/), so you can build, run and install it using cargo.

# Contributing

Feel free to open any pull request with improvements, suggestions or feature ideas.

![Meme](./assets/automation-meme.jpg)
