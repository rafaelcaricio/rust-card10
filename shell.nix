{ pkgs ? import <nixpkgs> {} }:

with pkgs;
with import ./default.nix;

stdenv.mkDerivation {
  name = "env";
  buildInputs = [
    gdb
    glibc_multi
    rust
    pkgsCross.arm-embedded.stdenv.cc
    openocd
  ];

  LIBCLANG_PATH="${llvmPackages.libclang}/lib"
  shellHook = ''
    echo "Run 'cargo build --release'"
  '';
}
