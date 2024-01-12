<!-- markdownlint-disable -->
<div align="center">
    <img src="https://github.com/KasarLabs/brand/blob/main/projects/deoxys/logo.png?raw=true" height="200" style="border-radius: 15px;">
</div>
<div align="center">
<br />
<!-- markdownlint-restore -->

[![Project license](https://img.shields.io/github/license/kasarLabs/deoxys.svg?style=flat-square)](LICENSE)
[![Pull Requests welcome](https://img.shields.io/badge/PRs-welcome-ff69b4.svg?style=flat-square)](https://github.com/kasarLabs/deoxys/issues?q=is%3Aissue+is%3Aopen+label%3A%22help+wanted%22)
<a href="https://twitter.com/KasarLabs">
<img src="https://img.shields.io/twitter/follow/KasarLabs?style=social"/> </a>
<a href="https://github.com/kasarlabs/deoxys">
<img src="https://img.shields.io/github/stars/kasarlabs/deoxys?style=social"/>
</a>

</div>

# 👽 Deoxys: Starknet full node client on Substrate

Deoxys is a Starknet *full node* client implementation using rust and leveraging the power of substrate and its community.

## ⚙️ Getting started

> ⚠️ We strongly recommend you use [nix](https://nixos.org/download) to set up your development environment.

### Setup with Nix

You will need to have [nix](https://nixos.org/download) installed on your system for this step, with [nix flakes](https://nixos.wiki/wiki/Flakes) enabled as an experimental feature. 
Then, simply run the following command inside the cloned repository:

```sh
nix develop
```

This will open a new shell session with all the required dependencies. Note that this is not a containerized environment, and local changes to files will persist.

### Manual setup

Make sure to have the following packages installed on your system:
```sh
openssl pkgconfig protobuf clang rocksdb alsa
```

You will also need to have [rust](https://www.rust-lang.org/) configured locally.

## 🧪 Testing

> ⚠️ Testing will require you both compile and run a test node locally. Make sure to have at least 300GB available to store the entirety of the Starknet blockchain history. **This will take time, so do it as soon as possible**.

Testing is done using the [ditto](https://github.com/KasarLabs/ditto) unit testing framework. Refer to that repository's documentation for further information

## 👍 Contribute

You are welcome to propose your help for contributions via [onlydust](https://app.onlydust.com/p/deoxys). Contributions will only be accepted via pull request through forks of this repository. Any pull request should be formatted using `cargo fmt` and linted with `cargo clippy` and *will be checked* during integration tests.

## 🤝 Partnerships

To establish a partnership with the Kasar team, or if you have any suggestion or
special request, feel free to reach us on [telegram](https://t.me/kasarlabs).

## ⚠️ License

Copyright (c) 2022-present, with the following
[contributors](https://github.com/KasarLabs/deoxys/graphs/contributors).

Deoxys is open-source software licensed under the
[Apache-2.0 License](https://github.com/KasarLabs/deoxys/blob/main/LICENSE).
