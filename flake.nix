{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = inputs: with inputs;
    utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit overlays system; };
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          name = "gambyte";
          src = pkgs.lib.cleanSource ./.;

          nativeBuildInputs = with pkgs; [ protobuf ];

          cargoLock.lockFile = ./Cargo.lock;
          cargoBuildFlags = "--release";
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            (rust-bin.stable.latest.default.override {
              targets = ["wasm32-unknown-unknown"];
            })
            gcc
            gdb
            wabt
          ];
        };
      }
    );
}
