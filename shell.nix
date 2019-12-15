with import <nixpkgs> {};
stdenv.mkDerivation {
  name = "dev";
  buildInputs = [ rustup ];
}


