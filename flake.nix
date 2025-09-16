{
  description = "Flake for pixelfix";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };

        buildInputs =
          with pkgs;
          [
            rustc
            cargo
            gcc
          ]
          ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
            pkgs.darwin.apple_sdk.frameworks.CoreFoundation
            pkgs.darwin.apple_sdk.frameworks.Security
          ];

        nativeBuildInputs = with pkgs; [
          pkg-config
        ];
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "pixelfix";
          version = "0.1.4";
          src = ./.;
          cargoHash = "sha256-sumQYwlezHIcUb4a039tnvYvCvu7ttyaCeay8OH9J5w=";

          inherit buildInputs nativeBuildInputs;

          buildType = "release";
          doCheck = true;

          meta = with pkgs.lib; {
            description = "Fix transparent pixels in PNG images by filling them with the nearest non-transparent pixel's color";
            homepage = "https://codeberg.org/hamhim/pixelfix-rs";
            license = licenses.mit;
            maintainers = [ "hamhimdev <hamhimdev@proton.me>"];
            mainProgram = "pixelfix";
            platforms = platforms.all;
          };
        };

        packages.pixelfix = self.packages.${system}.default;

        apps.default = flake-utils.lib.mkApp {
          drv = self.packages.${system}.default;
          name = "pixelfix";
        };

        apps.pixelfix = self.apps.${system}.default;

        formatter = pkgs.nixpkgs-fmt;
      }
    );
}
