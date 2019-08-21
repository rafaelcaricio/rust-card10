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
  epic-stubs = stdenv.mkDerivation {
    name = "epic-stubs";
    src = ./c;
    buildInputs = [ gcc python3 ];
    buildPhase = ''
      ${python3}/bin/python epicardium/api/genapi.py -H epicardium/epicardium.h -c client.c -s server.c
    '';
    installPhase = ''
      mkdir $out
      cp client.c server.c $out/
    '';
  };
  firmware = rustPlatform.buildRustPackage rec {
    name = "rust-card10";
    version = "0.0.0";
    src = ./.;
    cargoSha256 = "10qv30p3kr570glnyn37b6r8pgx48zj0mr9qf84m4wk4sjp3wxsd";
    buildInputs = [ pkgsCross.armhf-embedded.stdenv.cc ];
    prePatch = ''
      cp ${epic-stubs}/client.c l0dable/src/
    '';
    preBuild = ''
      export LIBCLANG_PATH=${llvmPackages.libclang}/lib
      export CPATH=${glibc_multi.dev}/include
      export CARGO_HOME=$(mktemp -d cargo-home.XXX)
      cd example
    '';
    doCheck = false;
    installPhase = ''
      mkdir -p $out/lib
      cp target/thumbv7em-none-eabihf/release/example $out/lib/example.elf
    '';
  };
in {
  inherit rust rustPlatform firmware epic-stubs;
}
