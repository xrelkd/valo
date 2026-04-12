{
  rustToolchain,
  cargoArgs,
  unitTestArgs,
  pkgs,
  ...
}:

let
  cargo-ext = pkgs.callPackage ./cargo-ext.nix { inherit cargoArgs unitTestArgs; };
in
pkgs.mkShell {
  name = "dev-shell";

  nativeBuildInputs = with pkgs; [
    cargo-ext.cargo-build-all
    cargo-ext.cargo-clippy-all
    cargo-ext.cargo-doc-all
    cargo-ext.cargo-nextest-all
    cargo-ext.cargo-test-all
    cargo-nextest
    rustToolchain

    jq

    hclfmt
    nixfmt
    prettier
    shfmt
    taplo
    treefmt

    shellcheck

    libgit2
    pkg-config
  ];

  shellHook = ''
    export NIX_PATH="nixpkgs=${pkgs.path}"

    # This allows the compiled build-script-build to find libgit2 at runtime
    export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath [ pkgs.libgit2 ]}:$LD_LIBRARY_PATH"
  '';
}
