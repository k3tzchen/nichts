{
  description = "nichts is a nix profile wrapper with pacman-like syntax";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [ cargo rustc ];
        };

        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "nichts";
          version = "0.2.39";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;

          meta = {
            description = "nichts is a nix profile wrapper with pacman-like syntax";
            homepage = "https://github.com/k3tzchen/nichts";
            license = pkgs.lib.licenses.mit;
            maintainers = with pkgs.lib.maintainers; [ ];
            mainProgram = "nichts";
          };
        };
      }
    );
}
