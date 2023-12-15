# rust-bible

## Getting started

### Installation

To install rust, we recommend using a tool called rustup. To download rustup and install rust
use the following command.

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Checking the version of rust you are running.

```
$ rustc --version
rustc 1.76.0-nightly (d86d65bbc 2023-12-10)
```

Rust has a 6-week(Beta) release process so there are many builds of Rust available at any time.
You have to update the version of rust you are running with `rustup update`.

### Switching the release channels

To switch between release channels use the following command.

```
rustup override set [nightly | beta | stable]
```
