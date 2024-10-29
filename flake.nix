{
  inputs = {
    flake-utils = {
      url = "github:numtide/flake-utils";
    };

    nixpkgs = {
      url = "github:nixos/nixpkgs/nixos-unstable";
    };
  };

  outputs =
    {
      self,
      flake-utils,
      nixpkgs,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };

      in
      {
        devShell = pkgs.mkShell {
          shellHook =
            if system == "aarch64-darwin" then
              ''
                export NIX_CFLAGS_COMPILE="-isystem ${pkgs.libelf}/include"
                export NIX_CFLAGS_COMPILE_FOR_TARGET=""
              ''
            else
              "";

          buildInputs = with pkgs; [
            libelf
            llvmPackages.clang
            pkg-config
            zlib
            zstd

            pkgsCross.avr.buildPackages.gcc11
          ];

          LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
        };
      }
    );
}
