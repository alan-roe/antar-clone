{
  description = "Let Me Talk, a web application to facilitate dialogue between parts of Self";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = { self, nixpkgs, fenix }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
        config.allowUnfree = true;
      };
    in {
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          (with fenix.packages.${system}; with stable; combine [
            cargo rustc rust-src rustfmt clippy rust-analyzer targets.wasm32-unknown-unknown.stable.rust-std
          ])
          dioxus-cli
          nodejs_20
          nodePackages_latest.tailwindcss
        ];
      };
    };
}
