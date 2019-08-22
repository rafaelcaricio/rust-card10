# Rust support for the card10 CCCamp15 badge

## Usage

You need arm-none-eabi-gcc and Rust nightly. Use rustup or NixOS.

```shell
rustup update nightly
rustup override set nightly
rustup target add thumbv7em-none-eabi
```
When cloning use `--recursive` to get the submodules, otherwise update them
afterwards:

```shell
git submodule update --init --recursive
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
| example  | l0dable example                                           |
| l0dable  | Helpers for building l0dables                             |

## TODO

- arkanoid
- [ ] alloc
