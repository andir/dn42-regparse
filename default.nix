{ pkgs ? import <nixpkgs> {} }:
pkgs.stdenv.mkDerivation {
  pname = "dn42-regparse";
  version = "0.0git";

  src = ./.;

  buildInputs = [ pkgs.rustc pkgs.cargo ];

  buildPhase = ''
    cargo build --locked --offline --release -j $NIX_BUILD_CORES --bin roagen
  '';

  installPhase = ''
    mkdir -p $out/bin
    cp -rv target/release/roagen $out/bin
  '';
}
