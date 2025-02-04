{nixpkgs ? <nixpkgs>}: let
  pkgs = import nixpkgs {};
in {
  default = pkgs.callPackage ./build.nix {};
  linux-x86 = pkgs.callPackage ./build.nix {pkgs = pkgs.pkgsCross.gnu64;};
  linux-arm = pkgs.callPackage ./build.nix {pkgs = pkgs.pkgsCross.aarch64-multiplatform;};
  windows-x86 = pkgs.callPackage ./build.nix {pkgs = pkgs.pkgsCross.mingwW64;};
  windows-arm = pkgs.callPackage ./build.nix {pkgs = pkgs.pkgsCross.ucrtAarch64;};
  macos-x86 = pkgs.callPackage ./build.nix {pkgs = pkgs.pkgsCross.x86_64-darwin;};
  macos-arm = pkgs.callPackage ./build.nix {pkgs = pkgs.pkgsCross.aarch64-darwin;};
}
