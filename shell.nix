let
  mozillaOverlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  pkgs = import <nixpkgs> { overlays = [ mozillaOverlay ]; };
in
with pkgs;
let
  openocd = callPackage ./openocd.nix {};
  rust = rustChannelOfTargets "nightly" null [ "thumbv7em-none-eabihf" ];
in
stdenv.mkDerivation {
  name = "env";
  buildInputs = with rustPlatform.rust; [
    gdb
    rust
  ];

  # Set Environment Variables
  RUST_BACKTRACE = 1;

  shellHook = ''
    echo "Starting openocd…"
    ${openocd}/bin/openocd-card10 &

    # Let openocd output scroll by
    sleep 1

    echo "Run 'cargo run --release'"
  '';
}
