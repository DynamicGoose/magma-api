{
  description = "Magma-API dev shell";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

  outputs =
    { self, nixpkgs }:
    let
      eachSystem = nixpkgs.lib.genAttrs nixpkgs.lib.systems.flakeExposed;
      pkgsFor = eachSystem (
        system:
        import nixpkgs {
          inherit system;
        }
      );
    in
    {
      devShells = eachSystem (
        system:
        let
          pkgs = pkgsFor.${system};
        in
        {
          default =
            let
              libPath =
                with pkgs;
                lib.makeLibraryPath [
                  libGL
                  libxkbcommon
                  wayland
                  vulkan-tools
                  vulkan-loader
                  alsa-lib
                ];
            in
            pkgs.mkShell {
              nativeBuildInputs = with pkgs; [
                rustc
                cargo
                gcc
                lld
                rust-analyzer
                rustfmt
                clippy
                nixfmt-tree
                nixfmt
                nil
              ];

              buildInputs = with pkgs; [
                pkg-config
              ];

              LD_LIBRARY_PATH = "${libPath}";
              RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
            };
        }
      );
    };
}

