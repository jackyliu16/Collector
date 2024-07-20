# Ref: https://github.com/fufexan/website/blob/master/nix/dev.nix
{ inputs, pkgs, ... }: 

let 
  themeName = (builtins.fromTOML (builtins.readFile "${inputs.theme}/theme.toml")).name;
  lib = pkgs.lib;
in pkgs.mkShellNoCC rec {
  name = "Collector";

  packages = with pkgs; [
    zola-with-ch-index    # TODO: maybe we could just only use overlays to cover zola and use zola
    zellij  # ~ tmux
    nodePackages.gramma
  ];

  shellHook = ''
    echo -e "\n\033[1;36m❄️ Welcome to the '${name}' devshell ❄️\033[0m\n"
    echo -e "\033[1;33m[Packages]\n\033[0m"
    echo -e "${lib.concatMapStringsSep "\n" (p: "  ${lib.getName p} \t- ${p.meta.description or ""}") packages}" | ${lib.getExe pkgs.unixtools.column} -ts $'\t'
    echo

    mkdir -p themes
    if [[ -d themes/${themeName} ]]; then
      true
    else
      ln -sn "${inputs.theme}" "themes/${themeName}"
      cp -r  --no-preserve=mode "${inputs.theme}/content" "."
      cp -r  --no-preserve=mode "${inputs.theme}/sass" "."
      cp -r  --no-preserve=mode "${inputs.theme}/static" "."
      cp -r  --no-preserve=mode "${inputs.theme}/templates" "."
      cp -ri --no-preserve=mode "${inputs.theme}/config.toml" "."
      cp -ri --no-preserve=mode "${inputs.theme}/config.toml.example" "."
      cp -ri --no-preserve=mode "${inputs.theme}/theme.toml" "."
    fi
  '';
}
