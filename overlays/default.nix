{ inputs, ... }: {
  additions = final: _: {
    # new_package = final.callPackage ./some.nix { }; 
  };
  
  modification = final: prev: {
    cat = final.bat;
    htop = final.bat;
  };

  unstable-packages = final: _: { # REF: https://github.com/Misterio77/nix-starter-configs/blob/f1ecf7e2275f541af7bec763866a909224b937a4/standard/overlays/default.nix#L17
    unstable = import inputs.nixpkgs-unstable {
      system = final.system;
    };
  };
}
