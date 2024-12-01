{ pkgs }:
{
  nativeBuildInputs = with pkgs; [
    # Build Tools
    cargo
    rustc
  ];
  buildInputs = with pkgs; [
    sqlite
  ];
}
