# Rust support for the card10 CCCamp19 badge

## Prerequisites

You need rust nightly and a working setup to compile the card10
firmware including the matching libc.

1) For instructions how to setup rust please see `https://rustup.rs`.

   Please ensure that you installed the latest rust nightly toolchain
   and add the `thumbv7em-none-eabi` target.

   ```shell
   rustup toolchain install nightly
   rustup update
   rustup target add thumbv7em-none-eabi --toolchain nightly
   ```

2) For instructions how to setup the card10 firmware check the dependency
   chapter in `https://firmware.card10.badge.events.ccc.de/how-to-build.html`.

3) Additionally you may need the packages for the llvm and libc i386
   dev headers.

4) Clone this repository with `--recursive` to get the submodules,
   otherwise update them afterwards:

   ```shell
   git submodule update --init --recursive
   ```

## Setup a Rust enabled firmware

To allow rust based apps on card10 you need a firmware which allows
to run custom elf binaries on the core. This requires a custom build
with `-Djailbreak_card10=true` as bootstrapping flag.

Assuming that you installed all required dependencies mentioned in
`https://firmware.card10.badge.events.ccc.de/how-to-build.html` this
should work as following:

```shell
cd c/
./bootstrap -Djailbreak_card10=true
ninja -C build/
```

And then copy `build/pycardium/pycardium_epicardium.bin` as
`card10.bin` onto your badge.

## Build and run Rust loadables

```shell
cargo +nightly build --release --target thumbv7-none-eabi
```

Then copy the resulting executable from the target directory 
`target/thumbv7em-none-eabi/release/example` into the
`apps` directory of your badge.

**Attention**: Its necessary to rename the executable to add the `elf`
extension (e.g `example` must be renamed as `example.elf`).

## Crates

| Crate    | Description                                               |
| ----     | ---                                                       |
| l0dable  | Helper crate for building l0dables                        |
| example  | l0dable example                                           |
| rkanoid  | Arkanoid clone                                            |


## Misc

### How to update the firmware bindings

1) Update the `c/` submodule to the latest firmware state.

2) Rebuild the firmware as described above.

3) Run the following script from the project root directory

   ```shell
   python c/epicardium/api/genapi.py -H c/epicardium/epicardium.h -c l0dable/src/client.c -s l0dable/src/server.c
   ```

4) Rebuild your app :)

## TODO

- [ ] alloc
