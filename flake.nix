{
  description = "a development shell for `shear`";

  inputs = {
    nixpkgs.url     = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    crane           = {
      url           = "github:ipetkov/crane";
      inputs        = { nixpkgs.follows = "nixpkgs"; };
    };
    rust-overlay    = {
      url           = "github:oxalica/rust-overlay";
      inputs        = { nixpkgs.follows = "nixpkgs"; };
    };
  };

 outputs = { crane, flake-utils, nixpkgs, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        # use the rust roolchain specified in the `rust-toolchain` file.
        overlays  = [ (import rust-overlay) ];
        pkgs      = import nixpkgs { inherit system overlays; };
        toolchain = pkgs.rust-bin.stable.latest.default;
        craneLib  = (crane.mkLib pkgs).overrideToolchain toolchain;
        src       = craneLib.cleanCargoSource ./.;
      in with pkgs; with pkgs.lib; let
        shear = (craneLib.buildPackage {
          inherit src;
          buildInputs       = [ ];
          nativeBuildInputs = [ ];
          meta              = {
            description     = "a library for trimming excess contents from things.";
            license         = [ licenses.mit ];
          };
        });
      in {
        packages          = { inherit shear; };
        devShells.default = craneLib.devShell {
          packages        = [ pkgs.just shear ];
          inputsFrom      = [ shear ];
          shellHook       = ''
            # tell rust-analyzer where the `std` sources can be found.
            export RUST_SRC_PATH=${pkgs.rustPlatform.rustLibSrc}
          '';
        };
      }
    );
}
