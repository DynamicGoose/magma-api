{
  description = "Magma-API dev shell";

  inputs = {
    dot-files.url = "git+https://codeberg.org/DynamicGoose/dot-files.git";
  };

  outputs =
    { self, dot-files }:
    {
      devShells = dot-files.lib.eachSystem (pkgs: {
        default =
          let
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
          };
      });
    };
}
