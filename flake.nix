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
          version = "0.2.99"; # Note: must be kept in sync with Cargo.lock
          hash = "sha256-1AN2E9t/lZhbXdVznhTcniy+7ZzlaEp/gwLEAucs6EA=";
          cargoHash = "sha256-DbwAh8RJtW38LJp+J9Ht8fAROK9OabaJ85D9C/Vkve4=";
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
          pname = "leptos-chartistry-workspace";
          version = "0.0.1";
          inherit src;
          strictDeps = true;
          CARGO_PROFILE = "release";
        };
        commonWasmArgs = commonArgs // {
          wasm-bindgen-cli = wasm-bindgen-cli-local;
          CARGO_BUILD_TARGET = "wasm32-unknown-unknown";
          # Cannot run `cargo test` on wasm
          doCheck = false;
        };

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;
        wasmArtifacts = craneLib.buildDepsOnly commonWasmArgs;

        demo = craneLib.buildTrunkPackage (
          commonWasmArgs
          // {
            pname = "chartistry-demo";
            cargoArtifacts = wasmArtifacts;
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

        # Build SSR example
        ssrExampleBin = craneLib.buildPackage (
          commonArgs
          // {
            pname = "chartistry-ssr-example-bin";
            inherit src cargoArtifacts;
            cargoExtraArgs = "-p my_example_ssr --bin=my_example_ssr --no-default-features --features=ssr";
          }
        );
        ssrExampleLib = craneLib.buildPackage (
          commonWasmArgs
          // {
            pname = "chartistry-ssr-example-lib";
            inherit src;
            cargoArtifacts = wasmArtifacts;
            cargoExtraArgs = "-p my_example_ssr --lib --no-default-features --features=hydrate";
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
          # Ensure we can build all code
          inherit demo ssrExampleBin ssrExampleLib;

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
