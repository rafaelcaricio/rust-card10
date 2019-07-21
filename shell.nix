{ pkgs ? import <nixpkgs> {} }:

with pkgs;
with import ./default.nix;

stdenv.mkDerivation {
  name = "env";
  buildInputs = [
    gdb
    rust
    pkgsCross.armhf-embedded.stdenv.cc
    py-crc16
  ];

  # Set Environment Variables
  RUST_BACKTRACE = 1;

  shellHook = ''
    echo "Starting openocd…"
    ${openocd}/bin/openocd-card10 &

    # Let openocd output scroll by
    sleep 1

    export PATH=`pwd`/.bin:$PATH
    echo "Run 'cargo card10 watchapp'"
  '';
}
