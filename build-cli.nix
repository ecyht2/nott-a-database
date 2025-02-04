{pkgs}: let
  inputs = import (builtins.path {
    path = ./inputs.nix;
    name = "nott-a-database";
  }) {inherit pkgs;};
  fs = pkgs.lib.fileset;
  # https://dev.to/misterio/how-to-package-a-rust-app-using-nix-3lh3
  manifest = (pkgs.lib.importTOML ./nott-a-database-cli/Cargo.toml).package;
in
  pkgs.rustPlatform.buildRustPackage {
    pname = manifest.name;
    version = manifest.version;
    src = fs.toSource {
      root = ./.;
      fileset =
        fs.intersection
        (fs.gitTracked ./.)
        (fs.unions [
          ./nott-a-database-core
          ./nott-a-database-cli
          ./nott-a-database
          ./Cargo.toml
          ./Cargo.lock
        ]);
    };
    buildAndTestSubdir = "nott-a-database-cli";

    cargoLock = {
      lockFile = ./Cargo.lock;
      outputHashes = {
        "refinery-0.8.14" = "sha256-WEH6qF0+ERdaEXtEMrfLXFAfePj2mBLOmjkhNONf9Hg=";
      };
    };

    inherit (inputs) buildInputs nativeBuildInputs;
  }
