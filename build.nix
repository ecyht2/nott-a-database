{ pkgs }:
let
  inputs = import (builtins.path { 
    path = ./inputs.nix;
    name = "nott-a-database";
  }) { inherit pkgs; };
in
pkgs.rustPlatform.buildRustPackage {
  name = "nott-a-database";
  src = builtins.path { 
    path = ./.;
    name = "nott-a-database";
  };
  
  cargoLock = {
    lockFile = builtins.path { 
      path = ./Cargo.lock;
      name = "nott-a-database";
    };
  };
  
  inherit (inputs) buildInputs;
}
