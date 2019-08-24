let
  mozillaOverlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  pkgs = import <nixpkgs> { overlays = [ mozillaOverlay ]; };
in
with pkgs;
let
  rust = rustChannelOfTargets "nightly" null [ "thumbv7em-none-eabi" ];
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
  l0dables = rustPlatform.buildRustPackage rec {
    name = "rust-card10";
    version = "0.0.0";
    src = ./.;
    cargoSha256 = "10nims5j9r0d7pcfbbj8ycqxhcx7n07958jvkib29b0sf9c6qh3z";
    buildInputs = [ pkgsCross.arm-embedded.stdenv.cc ];
    prePatch = ''
      cp ${epic-stubs}/client.c l0dable/src/
    '';
    NIX_DEBUG=1;
    LIBCLANG_PATH="${llvmPackages.libclang}/lib";
    CARGO_HOME="$(mktemp -d cargo-home.XXX)";
    preBuild = ''
      export CPATH="${glibc_multi.dev}/include:${stdenv.cc.cc}/lib/gcc/$(cc -dumpmachine)/${lib.getVersion pkgsCross.arm-embedded.stdenv.cc.cc}/include"
    '';
    doCheck = false;
    installPhase = ''
      mkdir -p $out/apps
      for f in target/thumbv7em-none-eabi/release/* ; do
        if [ -x $f ] && [ ! -d $f ] ; then
          cp $f $out/apps/$(basename $f).elf
        fi
      done
    '';
  };
in {
  inherit rust rustPlatform l0dables epic-stubs;
}
