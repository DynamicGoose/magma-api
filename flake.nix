{
  description = "Magma-API dev shell";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs =
    { self, nixpkgs }:
    let
      systems = [
        "x86_64-linux"
        "i686-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
    in
    {
      devShells = nixpkgs.lib.genAttrs systems (system: {
        default =
          let
            pkgs = import nixpkgs { inherit system; };
            libPath =
              with pkgs;
              lib.makeLibraryPath [
                libGL
                libxkbcommon
                wayland
                alsa-lib
              ];
          in
          pkgs.mkShell {
            nativeBuildInputs = with pkgs; [
              rustc
              cargo
              gcc
              rust-analyzer
              rustfmt
              clippy
            ];

            buildInputs = [ pkgs.pkg-config ];

            LD_LIBRARY_PATH = "${libPath}";
            RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

            shellHook = ''
              export IN_NIX_DEVELOP=1
              export NIX_ENV_NAME=magma-api
              ${pkgs.zsh}/bin/zsh
              exit
            '';
          };
      });
    };
}
