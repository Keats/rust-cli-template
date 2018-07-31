# rust-cli-template

A template to get started with writing cross-platforms CLI applications
hosted on GitHub.

## Features

- CLI app setup
- CI setup
- Cross-platform binary release via [trust](https://github.com/japaric/trust)
- Completion scripts setup
- Various repo housekeeping: changelog, editorconfig etc

## Pre-requisites
If you want to enable the publication of binaroes, you will need to do the following first.

### Get a GitHub token
Go to https://github.com/settings/tokens/new and create a `public_repo` token.

### Travis
Encrypt the token by using the `travis` gem:

```bash
$ travis encrypt $TOKEN
```

Copy the output somewhere, we will need it later!

## AppVeyor
Go to https://ci.appveyor.com/tools/encrypt and encrypt the same GitHub token or another one
with the same `public_repo` permission. You will also need that later!

## Get started
This is a [kickstart template](https://github.com/Keats/kickstart) so you
will need to have that installed:

```bash
$ cargo install kickstart
```

Once you have it installed and have the encrypted tokens generated above if needed, run:

```bash
$ kickstart https://github.com/Keats/rust-cli-template
```


## Steps left for you

- Check that the name repo name in the badges in the README match the actual one


## Template TODOs

- Is there an equivalent to Trust for GitLab?
