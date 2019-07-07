{ python3Packages }:

python3Packages.buildPythonPackage rec {
  pname = "crc16";
  version = "0.1.1";
  src = python3Packages.fetchPypi {
    inherit pname version;
    sha256 = "15nkx0pa4lskwin84flpk8fsw3jqg6wic6v3s83syjqg76h6my61";
  };
}
