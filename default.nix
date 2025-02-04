{nixpkgs ? <nixpkgs>}: let
  pkgs = import nixpkgs {};
in {
  default = pkgs.callPackage ./build-cli.nix {};
  cli-x86 = pkgs.callPackage ./build-cli.nix {pkgs = pkgs.pkgsCross.gnu64;};
  cli-arm = pkgs.callPackage ./build-cli.nix {pkgs = pkgs.pkgsCross.aarch64-multiplatform;};
}
