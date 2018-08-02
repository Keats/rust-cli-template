# rust-cli-template

A template to get started with writing cross-platforms CLI applications
hosted on GitHub.

## Features

- CLI app setup with clap
- Optional terminal colours & styling
- CI setup
- Cross-platform binary release via [trust](https://github.com/japaric/trust)
- Shell completion scripts setup
- Various repo housekeeping: changelog, editorconfig etc

All of the above is optional: you can pick only the features you want.

## Pre-requisites
If you want to enable the publication of binaries, you will need to do the following steps first.

### Create the GitHub repo
You might also need to enable it in Travis/AppVeyor.

### Encrypt a GH token for Travis
Go to https://github.com/settings/tokens/new and create 1 `public_repo` tokens, the
name is up to you but should be something like `{{project}} Travis` so it is easy to find.

Encrypt the token by using the `travis` gem:

```bash
$ travis encrypt $TOKEN -r $GH_USER/$GH_REPO
```

Copy the output somewhere, we will need it later!

### Encrypt it for AppVeyor
Go to https://github.com/settings/tokens/new and create 1 `public_repo` tokens, the
name is up to you but should be something like `{{project}} AppVeyor` so it is easy to find.

Go to https://ci.appveyor.com/tools/encrypt and encrypt the second GitHub token.
As before, you will also need the output later.

## Get started
This is a [kickstart template](https://github.com/Keats/kickstart) so you
will need to have `kickstart` installed:

```bash
$ cargo install kickstart
```

Once you have it installed and have the encrypted tokens generated above if needed, run:

```bash
$ kickstart https://github.com/Keats/rust-cli-template
```

## Steps left for you

- Check that the name repo name in the badges in the README match the actual one: it uses the project
name by default


## Template TODOs

- Is there an equivalent to Trust for GitLab?
