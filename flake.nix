{
  description = "flake for github:lavafroth/mustache2konsole";

  outputs =
    {
      self,
      nixpkgs,
      ...
    }:
    let
      forAllSystems =
        f:
        nixpkgs.lib.genAttrs nixpkgs.lib.systems.flakeExposed (system: f nixpkgs.legacyPackages.${system});
    in
    {
      packages = forAllSystems (pkgs: {
        default = pkgs.pkgsStatic.rustPlatform.buildRustPackage {
          pname = "mustache2konsole";
          version = "1.1.0";

          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
        };
      });

      devShells = forAllSystems (pkgs: {

        default = pkgs.mkShell {
          packages = with pkgs; [
            stdenv.cc.cc
          ];
        };

      });

      overlays.default = final: prev: {
        mustache2konsole = self.packages.${final.system}.default;
      };
    };
}
