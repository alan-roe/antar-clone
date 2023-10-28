{
  description = "Let Me Talk, a web application to facilitate dialogue between parts of Self";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-23.05";
    nixpkgs-unstable.url = "github:nixos/nixpkgs/nixos-unstable"; 
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = { self, nixpkgs, nixpkgs-unstable, fenix }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
        config.allowUnfree = true;
      };
      unstable = import nixpkgs-unstable {
          inherit system;
      };
    in {
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          (with fenix.packages.${system}; with stable; combine [
            cargo rustc rust-src rustfmt clippy rust-analyzer targets.wasm32-unknown-unknown.stable.rust-std
          ])
          unstable.dioxus-cli
          unstable.nodejs_20
          unstable.nodePackages_latest.tailwindcss
          # For Desktop stuff, stable pkgs because I don't know how to overlay a newer mesa version right now from this flake
          pkg-config
          gtk3
          webkitgtk_4_1
          libayatana-indicator
        ];
      };
    };
}
