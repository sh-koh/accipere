{
  description = "A neofetch alternative, written in haskell";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    haskell-flake.url = "github:srid/haskell-flake";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      imports = [
        inputs.haskell-flake.flakeModule
        inputs.treefmt-nix.flakeModule
      ];
      perSystem =
        {
          pkgs,
          self',
          config,
          ...
        }:
        let
          stack-wrapped = pkgs.symlinkJoin {
            name = "stack";
            paths = [ pkgs.stack ];
            buildInputs = [ pkgs.makeWrapper ];
            postBuild = ''
              wrapProgram $out/bin/stack \
                --add-flags "\
                  --nix \
                  --system-ghc \
                  --no-install-ghc \
                "
            '';
          };
        in
        {
          packages.default = self'.packages.accipere;
          haskellProjects.default = {
            basePackages = pkgs.haskellPackages;
            #basePackages = pkgs.haskell.packages.ghc9121;
            packages = {
              # Override dependencies
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
              hlsCheck.enable = false;
              tools =
                hp:
                {
                  inherit (hp)
                    cabal-install
                    haskell-language-server
                    hlint
                    ;
                  inherit (pkgs)
                    ghciwatch
                    ;
                  ghcid = null;
                  stack = stack-wrapped;
                  treefmt = config.treefmt.build.wrapper;
                }
                // config.treefmt.build.programs;
            };
          };
          treefmt.config = {
            projectRootFile = ".git/config";
            package = pkgs.treefmt;
            programs.nixfmt-rfc-style.enable = true;
            programs.cabal-fmt.enable = false;
            programs.hlint.enable = false;
          };
        };
    };
}
