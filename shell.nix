{ pkgs ? import <nixpkgs> {} }:
let
  inputs = import (builtins.path { 
    path = ./inputs.nix;
    name = "nott-a-database";
  }) { inherit pkgs; };
in
pkgs.mkShell {
  name = "nott-a-database";
  packages = with pkgs; [
    # Dev Tools
    clippy
    bacon
    bun
    rust-analyzer
    rustfmt
  ];
  inherit (inputs) buildInputs nativeBuildInputs;
}
