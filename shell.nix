let
  moz_overlay = import (builtins.fetchTarball
    "https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz");
in { pkgs ? import <nixpkgs> { overlays = [ moz_overlay ]; } }:
pkgs.mkShell rec {
  buildInputs = [
    pkgs.hello
    ((pkgs.rustChannelOf {
      date = "2021-02-28";
      channel = "nightly";
    }).rust.override {
      extensions = [ "rust-src" "rustfmt-preview" "clippy-preview" ];
    })
    pkgs.rust-analyzer
    pkgs.nixfmt
    # keep this line if you use bash
    pkgs.bashInteractive
    pkgs.pkgconfig
    pkgs.xorg.libX11
    pkgs.xorg.libXcursor
    pkgs.xorg.libXrandr
    pkgs.xorg.libXext
    ((pkgs.python3.withPackages (ps:
      with ps; [
        numpy
        python-language-server
        pyls-mypy
        grpcio
        grpcio-tools
      ])).override (args: { ignoreCollisions = true; }))
  ];
  LD_LIBRARY_PATH = with pkgs; stdenv.lib.makeLibraryPath buildInputs;
}
