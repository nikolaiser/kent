{
  description = "Description for the project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    make-shell.url = "github:nicknovitski/make-shell";
    rust-overlay.url = "github:oxalica/rust-overlay";

  };

  outputs =
    inputs@{
      flake-parts,
      make-shell,
      rust-overlay,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        make-shell.flakeModules.default
      ];
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
        "x86_64-darwin"
      ];
      perSystem =
        {
          pkgs,
          system,
          lib,
          ...
        }:
        let
          cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);

          rustPackage = pkgs.rust-bin.stable.latest.minimal;

          kent =
            (pkgs.makeRustPlatform {
              cargo = rustPackage;
              rustc = rustPackage;
            }).buildRustPackage
              {
                inherit (cargoToml.package) name version;
                src = ./.;
                cargoLock.lockFile = ./Cargo.lock;
                nativeBuildInputs = [ rustPackage ];
                buildInputs = lib.optional pkgs.stdenv.isDarwin [
                  pkgs.darwin.apple_sdk.frameworks.Security
                ];
              };
        in
        {
          _module.args.pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [ (import rust-overlay) ];
          };

          make-shells = {
            default = {
              buildInputs = [ rustPackage ];
            };

            test.buildInputs = [ kent ];
          };

          packages = {
            inherit kent;
            default = kent;
          };
        };
    };
}
