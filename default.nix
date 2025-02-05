{nixpkgs ? <nixpkgs>}: let
  pkgs = import nixpkgs {};
in {
  default = pkgs.callPackage ./build.nix {};
  cli-x86 = pkgs.callPackage ./build-cli.nix {pkgs = pkgs.pkgsCross.gnu64;};
  cli-arm = pkgs.callPackage ./build-cli.nix {pkgs = pkgs.pkgsCross.aarch64-multiplatform;};
  gui-x86 = pkgs.callPackage ./build.nix {pkgs = pkgs.pkgsCross.gnu64;};
  gui-arm = pkgs.callPackage ./build.nix {pkgs = pkgs.pkgsCross.aarch64-multiplatform;};
}
