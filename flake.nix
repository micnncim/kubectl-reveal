{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/release-23.05";

    flake-utils.url = "github:numtide/flake-utils";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    rust-overlay.url = "github:oxalica/rust-overlay";

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, flake-utils, crane, rust-overlay, advisory-db, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        buildInputs = with pkgs; [
          openssl
          pkg-config
        ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
          pkgs.libiconv
        ];

        craneLib = crane.lib.${system};

        src = pkgs.lib.cleanSourceWith {
          src = craneLib.path ./.;
          filter = pkgs.lib.cleanSourceFilter;
        };

        cargoArtifacts = craneLib.buildDepsOnly { inherit src buildInputs; };

        kubectl-reveal = craneLib.buildPackage { inherit cargoArtifacts src buildInputs; };

        rustToolchain = pkgs.rust-bin.beta.latest.default.override {
          extensions = [
            "rust-src"
            "rust-analyzer"
          ];
        };
      in
      with pkgs;
      {
        checks = {
          inherit kubectl-reveal;

          clippy = craneLib.cargoClippy {
            inherit cargoArtifacts src buildInputs;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          };

          doc = craneLib.cargoDoc { inherit cargoArtifacts src; };

          fmt = craneLib.cargoFmt { inherit src; };

          audit = craneLib.cargoAudit { inherit src advisory-db; };

          nextest = craneLib.cargoNextest {
            inherit cargoArtifacts src buildInputs;
            partitions = 1;
            partitionType = "count";
          };
        };

        packages = {
          inherit kubectl-reveal;
          default = kubectl-reveal;
        };

        apps.default = flake-utils.lib.mkApp { drv = kubectl-reveal; };

        devShells.default = mkShell {
          buildInputs = buildInputs
            ++ (with pkgs; [
            rustToolchain
          ]);

          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/src";
        };
      }
    );
}
