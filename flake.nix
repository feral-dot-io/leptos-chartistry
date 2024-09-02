{
  description = "Chartistry is a charting library for Leptos";
  inputs = {
    nixpkgs.url = "nixpkgs";
    utils.url = "flake-utils";
    crane.url = "github:ipetkov/crane";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      utils,
      rust-overlay,
      crane,
      advisory-db,
      ...
    }@inputs:
    utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system}.extend rust-overlay.overlays.default;
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          targets = [ "wasm32-unknown-unknown" ];
        };
        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        # Utilities
        wasm-bindgen-cli-local = pkgs.wasm-bindgen-cli.override {
          version = "0.2.93"; # Note: must be kept in sync with Cargo.lock
          hash = "sha256-DDdu5mM3gneraM85pAepBXWn3TMofarVR4NbjMdz3r0=";
          cargoHash = "sha256-birrg+XABBHHKJxfTKAMSlmTVYLmnmqMDfRnmG6g/YQ=";
        };

        # Build demo
        src =
          with pkgs;
          lib.cleanSourceWith {
            src = ./.;
            filter =
              path: type:
              (lib.hasSuffix ".html" path)
              || (lib.hasInfix "/assets/" path)
              || (craneLib.filterCargoSources path type);
          };

        commonArgs = {
          inherit src;
          pname = "leptos-chartistry-workspace";
          version = "0.0.1";
          CARGO_BUILD_TARGET = "wasm32-unknown-unknown";
          CARGO_PROFILE = "release";
        };

        cargoArtifacts = craneLib.buildDepsOnly (commonArgs // { doCheck = false; });

        demo = craneLib.buildTrunkPackage (
          commonArgs
          // {
            inherit cargoArtifacts;
            pname = "chartistry-demo";
            version = "0.0.1";
            strictDeps = true;
            wasm-bindgen-cli = wasm-bindgen-cli-local;

            cargoExtraArgs = "--package=demo";
            trunkExtraBuildArgs = "--public-url /leptos-chartistry";
            trunkIndexPath = "demo/index.html";
            # Create symlinks for each of our pages. Enables a static site.
            postInstall = ''
              ln -s index.html $out/examples.html
              mkdir -p $out/examples
              for f in demo/src/examples/*.rs; do
                f=''${f##*/} # Remove dir prefix
                f=''${f%.rs} # Remove file suffix
                f=''${f//_/-} # Replace underscores with dashes
                ln -s ../index.html $out/examples/$f.html
              done
            '';
          }
        );
      in
      {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            trunk
            wasm-bindgen-cli-local
          ];
        };

        checks = {
          inherit demo;

          clippy = craneLib.cargoClippy (
            commonArgs
            // {
              inherit cargoArtifacts;
              cargoClippyExtraArgs = "--all-targets -- --deny warnings";
            }
          );

          doc = craneLib.cargoDoc (
            commonArgs
            // {
              inherit cargoArtifacts;
              cargoDocExtraArgs = "--workspace";
            }
          );

          audit = craneLib.cargoAudit { inherit src advisory-db; };
          fmt = craneLib.cargoFmt commonArgs;
        };

        packages.demo = demo;
      }
    );
}
