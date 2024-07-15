{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";             # use eachDefaultSystem function
    call-flake.url = "github:divnix/call-flake";
    theme = {
      url = "github:isunjn/serene";
      flake = false;
    };
  };
  outputs = { self, call-flake, flake-utils, theme }@inputs: flake-utils.lib.eachDefaultSystem (system: let 
    parent-inputs = (call-flake ../.).inputs;
    pkgs = (call-flake ../.).outputs.pkgs.${system}.extend overlays;
    overlays = import ./overlays.nix { };
  in {
    packages.zola = pkgs.zola;
    packages.gramma = pkgs.nodePackages.gramma;
    packages.zola-with-ch-index = pkgs.zola-with-ch-index;

    devShell = import ./nix/shell.nix { inherit inputs pkgs; }; 
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
    ];
    extra-trusted-public-keys = [ # cache server public key
      "nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs="
    ];
  };
}
