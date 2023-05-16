<div align="center">
  <div>
    <h1>fetchfetch, but in rust</h1>
    <p>Fetch all of your fetches. written in <s>go</s> rust.</p>
    <img alt="License" src="https://img.shields.io/github/license/underthefoxtree/ffbir?style=flat-square">
    <img alt="File Count" src="https://img.shields.io/github/directory-file-count/underthefoxtree/ffbir?style=flat-square">
    <img alt="Github Stars" src="https://img.shields.io/github/stars/underthefoxtree/ffbir?style=flat-square">
    <img alt="GitHub Workflow Status" src="https://img.shields.io/github/actions/workflow/status/underthefoxtree/ffbir/rust.yml?branch=main&style=flat-square">
    <img alt="Screenshot" src="./media/screenshot.png">
</div>
</div>

## Installation
You can either download the latest [release](https://github.com/underthefoxtree/ffbir/releases) or build from source (recommended).

## Install from source (recommended)

### Requirements
[`rust`](https://www.rust-lang.org/) - building ffbir

[`just`](https://github.com/casey/just) (optional) - easy installation process

### Install with just
```bash
$ just install
```

### Install manually
```bash
# Build
$ cargo build --release
# Install
$ sudo cp target/release/ffbir /usr/bin
```

## Uninstall
If installed using a binary from the [releases](https://github.com/underthefoxtree/ffbir/releases) page, use your package manager to uninstall.

### Uninstall with just
```bash
$ just uninstall
```

### Uninstall manually
```bash
$ sudo rm /usr/bin/ffbir
```
