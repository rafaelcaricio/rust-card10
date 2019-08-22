# Rust support for the card10 CCCamp15 badge

## Prerequisites

You need Rust Nightly and the arm-none-eabi-gcc toolchain, including libc.

### Arch Linux

    sudo pacman -S arm-none-eabi-gcc arm-none-eabi-binutils arm-none-eabi-newlib

## Usage

You need Rust nightly. Use rustup or NixOS.

```shell
rustup update nightly
rustup override set nightly
rustup target add thumbv7em-none-eabi
```

Check out this repo's submodule (the C firmware).

```shell
cd example
cargo build --release
```

Then copy the resulting
`../target/thumbv7em-none-eabi/release/l0dable-example` to the badge
in USB Mass Storage mode in the `/apps/` subfolder. Don't forget to
rename with the `.elf` extension!

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
