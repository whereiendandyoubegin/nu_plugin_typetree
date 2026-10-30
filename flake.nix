{
  description = "nu_plugin_typetree";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
    let
      inherit (nixpkgs) lib;

      systems = [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" ];
      forAllSystems = f: lib.genAttrs systems (system: f nixpkgs.legacyPackages.${system});

      cargoToml = lib.importTOML ./Cargo.toml;

      mkTypetree = pkgs: pkgs.rustPlatform.buildRustPackage {
        pname = cargoToml.package.name;
        version = cargoToml.package.version;

        src = ./.;
        cargoLock.lockFile = ./Cargo.lock;

        nativeBuildInputs = [ pkgs.rustPlatform.bindgenHook ];
        buildInputs = lib.optionals pkgs.stdenv.hostPlatform.isDarwin [ pkgs.libiconv ];

        doCheck = false;

        meta.mainProgram = "nu_plugin_typetree";
      };
    in
    {
      packages = forAllSystems (pkgs: rec {
        nu_plugin_typetree = mkTypetree pkgs;
        default = nu_plugin_typetree;
      });

      devShells = forAllSystems (pkgs: {
        default = pkgs.mkShell {
          inputsFrom = [ (mkTypetree pkgs) ];
          packages = [ pkgs.rust-analyzer pkgs.clippy pkgs.rustfmt ];
        };
      });
    };
}
