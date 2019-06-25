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
  firmware = rustPlatform.buildRustPackage rec {
    name = "rust-card10";
    version = "0.0.0";
    src = ./.;
    cargoSha256 = "1jwg5x14g9m100yq7m21klmxgmd2gcsggpmcnxwa8vavmkf9gmrn";
    preBuild = "export CARGO_HOME=$(mktemp -d cargo-home.XXX)";
    doCheck = false;
    installPhase = ''
      mkdir -p $out/lib
      cp target/thumbv7em-none-eabihf/release/watchapp $out/lib/
    '';
  };
in {
  inherit openocd rust rustPlatform firmware;
}
