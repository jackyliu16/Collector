{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    nixpkgs-unstable.url = "github:nixos/nixpkgs?ref=master";   # use master branch of nixpkgs as unstable
    flake-utils.url = "github:numtide/flake-utils";             # use eachDefaultSystem function
  };

  outputs = { self
    , nixpkgs
    , nixpkgs-unstable
    , flake-utils
  }@inputs: flake-utils.lib.eachDefaultSystem (system: let 
    overlays = import ./overlays { inherit inputs; }; 
    # NOTE: use attrValues to convert attribute set into list(only Val) and append it into nixpkgs
    pkgs = nixpkgs.legacyPackages.${system}.appendOverlays (__attrValues overlays) ;
  in {
    pkgs = pkgs;
    overlays = overlays;

    devShell = pkgs.mkShellNoCC {
      buildInputs = with pkgs; [
        zellij
      ];
    };
  });

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
