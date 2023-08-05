{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/release-23.05";

    rust-overlay.url = "github:oxalica/rust-overlay";

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        rustToolchain = pkgs.rust-bin.beta.latest.default.override {
          extensions = [
            "rust-src"
            "rust-analyzer"
          ];
        };
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [
            openssl
            pkg-config
            rustToolchain
          ];

          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/src";
        };
      }
    );
}
