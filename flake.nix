{
  description = "AsterIDE Development Environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { nixpkgs, ... }:

    let
      forAllSystems = nixpkgs.lib.genAttrs nixpkgs.lib.systems.flakeExposed;
    in
    {
      devShells = forAllSystems (system:
        let
          pkgs = import nixpkgs { inherit system; };
        in
        rec {
          asteride = pkgs.mkShell {
            name = "asteride-dev-shell";

            buildInputs = with pkgs; [
              rustc
              cargo
              rustfmt
              clippy

              rust-analyzer
              pkg-config
              fmt
            ];
          };
          default = asteride;
        });

      packages = forAllSystems (system:
        let
          pkgs = import nixpkgs { inherit system; };
        in
        rec {
          asteride = pkgs.rustPlatform.buildRustPackage {
            pname = "asteride";
            version = "0.1.0";

            src = ./.;

            cargoLock = {
              lockFile = ./Cargo.lock;
            };
          };
          default = asteride;
        });

      formatter = forAllSystems (system:
        let
          pkgs = import nixpkgs { inherit system; };
        in
        pkgs.nixpkgs-fmt
      );
    };
}
