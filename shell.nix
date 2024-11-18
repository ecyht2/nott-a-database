{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
    name = "database-engineering-fyp";
    packages = with pkgs; [
    ];
    nativeBuildInputs = with pkgs; [
        sqlite
    ];
    buildInputs = with pkgs; [
        # Build Tools
        cargo
        rustc
        # Dev Tools
        clippy
        bacon
        bun
        rust-analyzer
        rustfmt
    ];
}
