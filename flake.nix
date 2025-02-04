{
  description = "A database for managing student results for University of Nottingham Malaysia Electrical and Electronics Engineering department.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs?ref=nixos-unstable";
  };

  outputs = {
    self,
    nixpkgs,
  }: let
    # The set of systems to provide outputs for
    allSystems = ["x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin"];
    # A function that provides a system-specific Nixpkgs for the desired systems
    forAllSystems = nixpkgs.lib.genAttrs allSystems;
    forAllPkgs = f:
      forAllSystems (system:
        f {
          pkgs = import nixpkgs {system = system;};
        });
  in {
    formatter = forAllSystems (system: nixpkgs.legacyPackages."${system}".alejandra);
    devShell = forAllPkgs ({ pkgs }: pkgs.callPackage ./shell.nix {});
  };
}
