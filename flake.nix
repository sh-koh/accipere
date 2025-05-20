{
  description = "A neofetch alternative, written in haskell";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    haskell-flake.url = "github:srid/haskell-flake";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
  };

  outputs =
    inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        inputs.haskell-flake.flakeModule
      ];

      systems = inputs.nixpkgs.lib.systems.flakeExposed;

      perSystem =
        { pkgs, self', ... }:
        {
          packages.default = self'.packages.accipere;
          haskellProjects.default = {
            basePackages = pkgs.haskellPackages;
            packages = {
              # Dependencies
              # aeson.source = "1.5.0.0";
            };
            settings = {
              # Settings for dependecies
              # aeson = {
              #   check = false;
              # };
            };
            devShell = {
              enable = true;
              # tools = hp: { cabal = hp.cabal; };
              hlsCheck.enable = false;
            };
          };
        };
    };
}
