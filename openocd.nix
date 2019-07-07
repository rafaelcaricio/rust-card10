{ stdenv, makeWrapper, openocd, fetchFromGitHub, autoreconfHook, git, which }:
let
  maxim-openocd = openocd.overrideAttrs (oa: {
      name = "maxim-openocd";
      src = fetchFromGitHub {
        owner = "maximmbed";
        repo = "openocd";
        rev = "e71ac88c9dbfa4ee1405d7a86376119dcc887ed1";
        sha256 = "18yc1wyclmjxqg6jilfcm60hi01pgqc4dilsmksqbhg23m6x4ycw";
        fetchSubmodules = true;
      };
      nativeBuildInputs = oa.nativeBuildInputs ++ [
        autoreconfHook
        git
        which
      ];
      enableParallelBuilding = true;
    });
  card10-scripts = stdenv.mkDerivation {
    name = "card10-scripts";
    src = ./c/openocd/scripts;
    dontBuild = true;
    installPhase = ''
      mkdir -p $out/share/openocd
      cp -ar . $out/share/openocd/scripts
   '';
  };
in
  stdenv.mkDerivation {
    name = "openocd-card10";
    src = maxim-openocd;
    phases = [ "unpackPhase" "installPhase" ];
    buildInputs = [ makeWrapper maxim-openocd card10-scripts ];
    installPhase = ''
      mkdir -p $out/bin
      makeWrapper ${maxim-openocd}/bin/openocd $out/bin/openocd-card10 \
        --add-flags "-f ${card10-scripts}/share/openocd/scripts/interface/cmsis-dap.cfg" \
        --add-flags "-f ${card10-scripts}/share/openocd/scripts/target/max32665.cfg"
    '';
  }
