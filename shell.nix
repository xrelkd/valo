with import <nixpkgs> { };

stdenv.mkDerivation {
  name = "valo-dev";

  RUST_BACKTRACE = 1;

  nativeBuildInputs = [ rustup ];
}
