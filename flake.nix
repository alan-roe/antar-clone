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
          (with fenix.packages.${system}; with latest; combine [
            cargo rustc rust-src rustfmt clippy rust-analyzer targets.wasm32-unknown-unknown.latest.rust-std
          ])
          unstable.trunk
          unstable.nodePackages_latest.tailwindcss
          unstable.cargo-nextest
          unstable.cargo-tarpaulin
          unstable.wasm-pack # for web tests
        ];
      };
    };
}
