let
  pkgs = import
    (builtins.fetchTarball {
      # TODO https://github.com/NixOS/nixpkgs/pull/217058
      url = "https://github.com/winterqt/nixpkgs/archive/90b4570ab50c8a5bc8fdc798084beb86c252229d.tar.gz";
    })
    { };

in
pkgs.mkShell {
  shellHook =
    if builtins.currentSystem == "aarch64-darwin" then
      ''
        export NIX_CFLAGS_COMPILE="-isystem ${pkgs.libelf}/include"
        export NIX_CFLAGS_COMPILE_FOR_TARGET=""
      ''
    else
      "";

  buildInputs = with pkgs; [
    clang
    iconv
    libelf
    pkg-config
    zlib

    pkgsCross.avr.buildPackages.gcc11
  ];

  LIBCLANG_PATH = "${pkgs.llvmPackages_11.libclang.lib}/lib";
}
