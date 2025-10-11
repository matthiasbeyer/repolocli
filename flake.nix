{
  description = "The repolocli framework";
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-25.05";
    unstable-nixpkgs.url = "nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    cargo-changelog = {
      url = "github:matthiasbeyer/cargo-changelog";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
        crane.follows = "crane";
        rust-overlay.follows = "rust-overlay";
      };
    };
  };

  outputs = inputs: inputs.flake-utils.lib.eachSystem [ "x86_64-linux" "aarch64-linux" ]
    (system:
      let
        pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [ (import inputs.rust-overlay) ];
        };

        unstable = import inputs.unstable-nixpkgs {
          inherit system;
        };

        nightlyRustTarget = pkgs.rust-bin.selectLatestNightlyWith (toolchain: pkgs.rust-bin.fromRustupToolchain { channel = "nightly"; components = [ "rustfmt" ]; });
        nightlyCraneLib = (inputs.crane.mkLib pkgs).overrideToolchain nightlyRustTarget;
        rustTarget = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        craneLib = (inputs.crane.mkLib pkgs).overrideToolchain rustTarget;

        tomlInfo = craneLib.crateNameFromCargoToml { cargoToml = ./Cargo.toml; };
        inherit (tomlInfo) version;
        pname = "repolocli";

        src =
          let
            nixFilter = path: _type: !pkgs.lib.hasSuffix ".nix" path;
            extraFiles = path: _type: !(builtins.any (n: pkgs.lib.hasSuffix n path) [ ".github" ".sh" ]);
            filterPath = path: type: builtins.all (f: f path type) [
              nixFilter
              extraFiles
              pkgs.lib.cleanSourceFilter
            ];
          in
          pkgs.lib.cleanSourceWith {
            src = ./.;
            filter = filterPath;
          };

        repolocliBuildInputs = [
          pkgs.cmake
          pkgs.gcc
          pkgs.openssl
          pkgs.pkg-config
        ];

        cargoArtifacts = craneLib.buildDepsOnly {
          inherit src pname;
          buildInputs = repolocliBuildInputs;
        };

        repolocli = craneLib.buildPackage {
          inherit cargoArtifacts src pname version;
          cargoExtraArgs = "--all-features -p repolocli";

          buildInputs = repolocliBuildInputs;
        };

        repolocli-docs = craneLib.cargoDoc {
          inherit cargoArtifacts src pname version;
          doInstallCargoArtifacts = true;
          RUSTDOCFLAGS = "-D warnings"; # Error out if there is a warning
        };

        rustfmt' = pkgs.writeShellScriptBin "rustfmt" ''
          exec "${nightlyRustTarget}/bin/rustfmt" "$@"
        '';

        customCargoMultiplexer = pkgs.writeShellScriptBin "cargo" ''
          case "$1" in
            +nightly)
              shift
              export PATH="${nightlyRustTarget}/bin/:''$PATH"
              exec ${nightlyRustTarget}/bin/cargo "$@"
              ;;
            *)
              exec ${rustTarget}/bin/cargo "$@"
          esac
        '';
      in
      rec {
        checks = {
          inherit repolocli;

          repolocli-clippy = craneLib.cargoClippy {
            inherit cargoArtifacts src pname;
            cargoClippyExtraArgs = "--benches --examples --tests --all-features -- --deny warnings";
          };

          repolocli-fmt = nightlyCraneLib.cargoFmt {
            inherit src pname;
          };

          repolocli-tests = craneLib.cargoNextest {
            inherit cargoArtifacts src pname;
            buildInputs = repolocliBuildInputs;
            cargoNextestExtraArgs = "--no-tests pass";
          };
        };

        packages = {
          default = packages.repolocli;
          inherit repolocli;
        };

        devShells = {
          default = devShells.repolocli;

          repolocli = pkgs.mkShell {
            buildInputs = repolocliBuildInputs ++ [
              pkgs.codespell
              pkgs.nodePackages.markdownlint-cli
            ];

            nativeBuildInputs = [
              rustfmt'
              rustTarget
              inputs.cargo-changelog.packages."${system}".changelog
              pkgs.openssl
              pkgs.cmake
              pkgs.gcc
              pkgs.pkg-config
            ];
          };
        };
      }
    );
}
