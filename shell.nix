{ pkgs ? import <nixpkgs> {} }:

with pkgs;
with import ./default.nix;

multiStdenv.mkDerivation {
  name = "env";
  buildInputs = [
    gdb
    rust
    pkgsCross.arm-embedded.stdenv.cc
    openocd
  ];

  # Set Environment Variables
  RUST_BACKTRACE = 1;

  shellHook = ''
    export LIBCLANG_PATH=${llvmPackages.libclang}/lib
    echo "Run 'cd example && cargo build --release'"
  '';
}
