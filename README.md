# Rust support for the card10 CCCamp15 badge

## Usage

You need Rust nightly. Use rustup or NixOS.

```shell
rustup update nightly
rustup target add thumbv7em-none-eabihf
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


# Talk

- rust on the rad10, on the r0ket
- prototype
- ideal: everything in safe idiomatic rust with nice interfaces
- too much work duplicating C effort
- firmware (max PAC, card10, watchapp)
- still need too learn many details

- day 0: start looking at epicardium/pycardium
- l0dables as most user-friendly way
- example/main.rs
- how built:
  - .cargo/config
  - l0dable/build.rs
  - l0dable/lib.rs
