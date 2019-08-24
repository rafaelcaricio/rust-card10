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
  ];

  LIBCLANG_PATH="${llvmPackages.libclang}/lib";
  shellHook = ''
    export CPATH="${glibc_multi.dev}/include:${stdenv.cc.cc}/lib/gcc/$(cc -dumpmachine)/${lib.getVersion pkgsCross.arm-embedded.stdenv.cc.cc}/include"
    echo "Run 'cargo build --release'"
  '';
}
