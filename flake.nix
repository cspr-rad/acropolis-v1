{
  description = "acropolis";

  nixConfig = {
    extra-substituters = [
      "https://crane.cachix.org"
      "https://nix-community.cachix.org"
    ];
    extra-trusted-public-keys = [
      "crane.cachix.org-1:8Scfpmn9w+hGdXH/Q9tTLiYAE/2dnJYRJP7kl80GuRk="
      "nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs="
    ];
  };

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    nixpkgs-r0vm.url = "github:NixOS/nixpkgs/nixos-unstable-small";
    treefmt-nix.url = "github:numtide/treefmt-nix";
    treefmt-nix.inputs.nixpkgs.follows = "nixpkgs";
    fenix.url = "github:nix-community/fenix";
    fenix.inputs.nixpkgs.follows = "nixpkgs";
    crane.url = "github:ipetkov/crane";
    crane.inputs.nixpkgs.follows = "nixpkgs";
    kurtosis.url = "github:marijanp/kurtosis/nixify";
    kurtosis.inputs.unstable.follows = "nixpkgs";
  };

  outputs = inputs@{ flake-parts, treefmt-nix, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" "x86_64-darwin" "aarch64-darwin" ];
      imports = [
        treefmt-nix.flakeModule
      ];
      perSystem = { self', inputs', pkgs, lib, ... }:
        let
          rustToolchain = inputs'.fenix.packages.latest.toolchain;
          craneLib = (inputs.crane.mkLib pkgs).overrideToolchain rustToolchain;

          rustup-mock = pkgs.writeShellApplication {
            name = "rustup";
            text = ''
              # the buildscript uses rustup toolchain to check
              # whether the risc0 toolchain was installed
              if [[ "$1" = "toolchain" ]]
              then
                printf "risc0\n"
              elif [[ "$1" = "+risc0" ]]
              then
                printf "${rustToolchain}/bin/rustc"
              fi
            '';
          };

          acropolisAttrs = rec {
            src = lib.cleanSourceWith {
              src = craneLib.path ./.;
              filter = craneLib.filterCargoSources;
            };
            nativeBuildInputs = with pkgs; [
              pkg-config
              cargo-risczero
              rustup-mock
            ];
            buildInputs = with pkgs; [
              openssl.dev
            ] ++ lib.optionals stdenv.isDarwin [
              libiconv
              darwin.apple_sdk.frameworks.SystemConfiguration
            ];
            checkInputs = [
              inputs'.nixpkgs-r0vm.legacyPackages.r0vm
            ];
            cargoVendorDir = craneLib.vendorMultipleCargoDeps {
              inherit (craneLib.findCargoFiles src) cargoConfigs;
              cargoLockList = [
                ./methods/guest/Cargo.lock
                ./Cargo.lock
                ./rust-std-Cargo.lock
              ];
            };
            preBuild = ''
              # The vendored cargo sources will be placed into .cargo-home,
              # however it seems that since the risc0_build crate
              # calls cargo at build time in this directory cargo will be
              # looking for .cargo
              mkdir .cargo
              mv .cargo-home/config.toml .cargo/config.toml
              export RISC0_RUST_SRC=${rustToolchain}/lib/rustlib/src/rust;
            '';
          };
        in
        {
          checks = {
            inherit (self'.packages) acropolis;
          };
          treefmt = {
            projectRootFile = ".git/config";
            programs.nixpkgs-fmt.enable = true;
            programs.rustfmt.enable = true;
            programs.rustfmt.package = craneLib.rustfmt;
            settings.formatter = { };
          };
          devShells.default = pkgs.mkShell {
            RISC0_RUST_SRC = "${rustToolchain}/lib/rustlib/src/rust";
            RISC0_DEV_MODE = 1;
            inputsFrom = [ self'.packages.acropolis ];
            packages = [
              inputs'.nixpkgs-r0vm.legacyPackages.r0vm
              pkgs.nodejs
              inputs'.kurtosis.packages.kurtosis
            ];
          };
          packages = {
            acropolis-deps = craneLib.buildDepsOnly (acropolisAttrs // {
              pname = "acropolis";
            });

            acropolis = craneLib.buildPackage (acropolisAttrs // {
              cargoArtifacts = self'.packages.acropolis-deps;
              meta.mainProgram = "client";
            });

            default = self'.packages.acropolis;

            acropolis-docs = craneLib.cargoDoc (acropolisAttrs // {
              cargoArtifacts = self'.packages.acropolis-deps;
            });
          };
        };
      flake = {
        herculesCI.ciSystems = [ "x86_64-linux" ];
      };
    };
}
