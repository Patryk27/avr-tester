{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    clang
    libelf
    pkg-config
    zlib

    pkgsCross.avr.buildPackages.gcc
  ];

  LIBCLANG_PATH = "${pkgs.llvmPackages_11.libclang.lib}/lib";
}
