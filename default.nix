let
  mozillaOverlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  pkgs = import <nixpkgs> { overlays = [ mozillaOverlay ]; };
in
with pkgs;
let
  openocd = callPackage ./openocd.nix {};
  rust = rustChannelOfTargets "nightly" null [ "thumbv7em-none-eabihf" ];
  rustPlatform = makeRustPlatform {
    rustc = rust;
    cargo = rust;
  };
  py-crc16 = callPackage ./python-crc16.nix {};
  firmware = rustPlatform.buildRustPackage rec {
    name = "rust-card10";
    version = "0.0.0";
    src = ./.;
    cargoSha256 = "04blshy2c4xms9v8ik921qs3ym4vq8rsx8pw78brw7r88x6lx01a";
    buildInputs = [ py-crc16 ] ++
      (with pkgsCross.armhf-embedded.stdenv; [
        cc binutils
      ]);
    preBuild = "export CARGO_HOME=$(mktemp -d cargo-home.XXX)";
    doCheck = false;
    installPhase = ''
      mkdir -p $out/lib
      cp target/thumbv7em-none-eabihf/release/watchapp $out/lib/

      arm-none-eabihf-objcopy -O binary $out/lib/watchapp $out/lib/card10.bin
      ${python3}/bin/python c/bootloader/crc_patch.py $out/lib/card10.bin
    '';
  };
in {
  inherit openocd rust rustPlatform firmware;
}
