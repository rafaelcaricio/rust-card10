# Rust support for the card10 CCCamp15 badge

## Usage

You need Rust nightly. Use rustup or NixOS.

```shell
rustup update nightly
rustup target add thumbv7em-none-eabihf
cd example
cargo build --release
```

## Crates

| Crate    | Description                                               |
| ----     | ---                                                       |
| card10   | Helpers for implementing custom firmwares                 |
| example  | l0dable example                                           |
| l0dable  | Helpers for building l0dables                             |
| max32665 | Peripheral Access Crate for implementing your own drivers |
| watchapp | Sample firmware                                           |

## TODO

- arkanoid
- [ ] alloc
