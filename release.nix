{ pkgs ? import <nixpkgs> {},
}:
with pkgs;

let
  cSrc = stdenv.mkDerivation {
    name = "card10-src";
    src = ./.;
    phases = [ "unpackPhase" "patchPhase" "installPhase" ];
    nativeBuildInputs = [ git ];
    prePatch = "cd c";
    postPatch = ''
      VERSION="$(git describe --always)"
      GITHASH="$(git rev-parse HEAD)"

      substituteInPlace tools/version-header.sh \
        --replace "\$VERSION" "$VERSION" \
        --replace "\$GITHASH" "$GITHASH"
    '';
    installPhase = ''
      cp -ar . $out
    '';
  };
  cFirmware = import "${cSrc}";
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
