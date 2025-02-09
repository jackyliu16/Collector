{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    systems.url = "github:nix-systems/default";
    rust-flake.url = "github:juspay/rust-flake";
    rust-flake.inputs.nixpkgs.follows = "nixpkgs";
    process-compose-flake.url = "github:Platonic-Systems/process-compose-flake";
    cargo-doc-live.url = "github:srid/cargo-doc-live";
    call-flake.url = "github:divnix/call-flake";

    # Dev tools
    treefmt-nix.url = "github:numtide/treefmt-nix";
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import inputs.systems;
      imports = [
        inputs.treefmt-nix.flakeModule
        inputs.rust-flake.flakeModules.default
        inputs.rust-flake.flakeModules.nixpkgs
        inputs.process-compose-flake.flakeModule
        inputs.cargo-doc-live.flakeModule
      ];
      perSystem = { config, self', pkgs, lib, ... }: {
        rust-project.crates."Collector".crane.args = {
          buildInputs = with pkgs; [
            glibc
            musl
            pkgsCross.mingw32.stdenv.cc
            pkgsCross.mingwW64.stdenv.cc
          ] ++ lib.optionals pkgs.stdenv.isDarwin (
            with pkgs.darwin.apple_sdk.frameworks; [
              IOKit
            ]
          );
        };

        # Add your auto-formatters here.
        # cf. https://nixos.asia/en/treefmt
        treefmt.config = {
          projectRootFile = "flake.nix";
          programs = {
            nixpkgs-fmt.enable = true;
            rustfmt.enable = true;
          };
        };

        devShells.default = pkgs.mkShell {
          inputsFrom = [
            self'.devShells.rust
            config.treefmt.build.devShell
          ];
          packages = [
            pkgs.cargo-watch
            config.process-compose.cargo-doc-live.outputs.package
          ];
        };
        packages.default = self'.packages.Collector;
      };
    };
    nixConfig = { # REF: https://nixos-and-flakes.thiscute.world/nix-store/add-binary-cache-servers
      substituters = [
        # cache mirror located in China
        # status: https://mirror.sjtu.edu.cn/
        "https://mirror.sjtu.edu.cn/nix-channels/store"
        # status: https://mirrors.ustc.edu.cn/status/
        # "https://mirrors.ustc.edu.cn/nix-channels/store"

        "https://cache.nixos.org"

        # nix community's cache server
        "https://nix-community.cachix.org"
        "https://zola.cachix.org"
      ];
      extra-trusted-public-keys = [ # cache server public key
        "nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs="
        "zola.cachix.org-1:NuHGH5vaZb05JjJIzx+rARDRys05gfoeJqIUSrS0VM4="
      ];
    };
}
