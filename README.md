# Rust support for the card10 CCCamp19 badge

## Prebuilt binaries

By courtesy of this Gitlab's CI system, and NixOS, we build `.elf`
files for you drop into the `apps/` directory of your card10 badge.

Because running ELF binaries requires a jailbroken base firmware
starting with v1.5, we build this one too. It includes this project's
Rust binaries.

https://git.card10.badge.events.ccc.de/astro/rust-card10/-/jobs

## Prerequisites

You need rust nightly and a working setup to compile the card10
firmware including the matching libc.

1) For instructions how to setup rust please see https://rustup.rs.

   Please ensure that you installed the latest rust nightly toolchain
   and add the `thumbv7em-none-eabi` target.

   ```shell
   rustup toolchain install nightly
   rustup update
   rustup target add thumbv7em-none-eabi --toolchain nightly
   ```

2) For instructions how to setup the card10 firmware check the dependency
   chapter in https://firmware.card10.badge.events.ccc.de/how-to-build.html.

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
https://firmware.card10.badge.events.ccc.de/how-to-build.html this
should work as following:

```shell
cd card10-sys/firmware
./bootstrap -Djailbreak_card10=true
ninja -C build/
```

And then copy `build/pycardium/pycardium_epicardium.bin` as
`card10.bin` onto your badge.

## Build and run Rust loadables

### Setup

If you want to come up with your own rust based loadable crate a few
preparations are required:

  - Setup the new crate repository.
 
  - Add `card10-l0dable = "^0.1"` as a dependency to your new crate.
 
  - Change the configuration of the default cargo release profile inside your
    `Cargo.toml` file:
 
    ```
    [profile.release]
    opt-level = "s"
    panic = "abort"
    ```

  - Create (or update) the `thumbv7em-none-eabi` target configuration at
    `$PROJECT/.cargo/config` with the following rustflags:
 
    ```
    [target.thumbv7em-none-eabi]
    rustflags = [
      "-C", "linker=arm-none-eabi-gcc",
      "-C", "link-args=-Tl0dable.ld -n -pie -fPIC",
      "-C", "relocation-model=pic",
    ]

    [build]
    target = "thumbv7em-none-eabi"
    ```

  - Ensure that your crate is marked as a `non_std` project and make
    `card10-l0dable` aware of your custom main function. This should require
    the following update to your `main.rs` file.

    ```main.rs
    #![no_std]
    #![no_main]

    use card10_l0dable::main;

    main!(main);
    fn main() {}
    ```

### Compilation

To compile the project use the nightly toolchain and define the proper target.

```shell
cargo +nightly build --release --target thumbv7em-none-eabi
```

### Transfer to card10

Then copy the resulting executable from the target directory 
`target/thumbv7em-none-eabi/release/example` into the
`apps` directory of your badge.

**Attention**: Its necessary to rename the executable to add the `elf`
extension (e.g `example` must be renamed as `example.elf`).

## Crates

| Crate           | Description                                               |
| ----            | ---                                                       |
| card10-sys      | Unsafe C bindings for l0dables                            |
| card10-alloc    | alloc::* support for l0dables                             |
| card10-l0dable  | High-level crate for building l0dables                    |
| example         | l0dable example                                           |
| rkanoid         | Arkanoid clone                                            |
| draw-image      | Example of drawing a static image to the display          |


## Misc

### How to update the firmware bindings

1) Update the `card10-sys/firmware` submodule to the latest firmware state.

2) Rebuild the firmware as described above.

3) Run the following script from the project root directory

   ```shell
   python card10-sys/firmware/epicardium/api/genapi.py -H card10-sys/firmware/epicardium/epicardium.h -c card10-sys/vendor/client.c -s card10-sys/vendor/server.c
   ```

4) Rebuild your app :)
