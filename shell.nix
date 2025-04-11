{ pkgs ? import <nixpkgs> {} }:
let
  libPath = with pkgs; lib.makeLibraryPath [
    libGL
    libxkbcommon
    wayland
    alsa-lib
    # xorg.libX11
    # xorg.libXcursor
    # xorg.libXi
    # xorg.libXrandr
  ];
in
with pkgs; mkShell {
  nativeBuildInputs = with pkgs; [
    rustc
    cargo
    gcc
    rust-analyzer
    rustfmt
    clippy
  ];

  buildInputs = [pkgs.pkg-config];

  LD_LIBRARY_PATH = "${libPath}";
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
