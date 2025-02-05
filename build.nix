{
  buildNpmPackage,
  bundle ? false,
  pkgs,
  stdenvNoCC,
}: let
  inputs = import ./inputs.nix {inherit pkgs;};
  fs = pkgs.lib.fileset;

  bundleFlags =
    if bundle
    then ""
    else "--no-bundle";

  # https://dev.to/misterio/how-to-package-a-rust-app-using-nix-3lh3
  manifest = (pkgs.lib.importTOML ./nott-a-database/src-tauri/Cargo.toml).package;
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
in
  stdenvNoCC.mkDerivation {
    inherit pname version src;

    configurePhase = ''
      runHook preConfigure
      pushd nott-a-database
      ln -s ${nodeDependencies}/lib/node_modules ./node_modules
      popd
      runHook postConfigure
    '';

    buildPhase = ''
      runHook preBuild
      pushd nott-a-database
      bun run tauri -- build ${bundleFlags}
      popd
      runHook postBuild
    '';

    installPhase = ''
      runHook preInstall
      pushd nott-a-database
      bun run tauri -- build ${bundleFlags}
      popd
      runHook postInstall
    '';

    inherit (inputs) buildInputs nativeBuildInputs;
  }
