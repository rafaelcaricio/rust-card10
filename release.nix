{ pkgs ? import <nixpkgs> {},
  jailbreak ? true,
}:
with pkgs;

let
  cSrc = stdenv.mkDerivation {
    name = "card10-src";
    src = ./c;
    phases = [ "unpackPhase" "patchPhase" "installPhase" ];
    patches = [
      ./0001-feat-nix-add-jailbreak-arg.patch
    ];
    installPhase = ''
      cp -ar . $out
    '';
  };
  cFirmware = import "${cSrc}" { inherit pkgs jailbreak; };
  rustL0dables = (import ./default.nix).l0dables;
  release = stdenv.mkDerivation {
    name = "card10-firmware";
    buildInputs = [ cFirmware rustL0dables ];
    phases = [ "installPhase" ];
    installPhase = ''
      mkdir $out
      cp -r ${cFirmware}/card10/* $out/
      chmod u+w $out/apps
      cp ${rustL0dables}/apps/* $out/apps/
    '';
  };
in release
