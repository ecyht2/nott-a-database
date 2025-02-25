{pkgs ? import <nixpkgs> {}}: let
  inputs = import (builtins.path {
    path = ./inputs.nix;
    name = "nott-a-database";
  }) {inherit pkgs;};
in
  pkgs.mkShell {
    name = "nott-a-database";
    packages = with pkgs; [
      # Dev Tools
      clippy
      bacon
      rust-analyzer
      rustfmt
    ];
    inherit (inputs) buildInputs nativeBuildInputs;
    # https://github.com/tauri-apps/tauri-docs/issues/1560
    shellHook = ''
      export XDG_DATA_DIRS=${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}:$XDG_DATA_DIRS
      export NOTT_A_DATABASE_LOG=debug
    '';
  }
