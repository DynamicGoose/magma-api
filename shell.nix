{ pkgs ? import <nixpkgs> { } }:
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
  inputsFrom = [ ];
  buildInputs = [ rustup pkg-config alsa-lib ];
  LD_LIBRARY_PATH = "${libPath}";
}