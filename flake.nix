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
    packages.zola = pkgs.zola;
    packages.htop = pkgs.htop;
  });
}
