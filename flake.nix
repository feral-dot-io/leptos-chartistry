{
  description = "Chartistry is a charting library for Leptos";
  inputs = {
    nixpkgs.url = "nixpkgs";
    utils.url = "flake-utils";
    crane.url = "github:ipetkov/crane";

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
    cargo-leptos-src = {
      url = "github:leptos-rs/cargo-leptos?tag=v0.2.44";
      flake = false; # Only provides a devShell
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    trunk-src = {
      url = "github:trunk-rs/trunk?tag=v0.21.14";
      flake = false; # Avoid breakage if added
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
        pkgs = nixpkgs.legacyPackages.${system}.appendOverlays [
          rust-overlay.overlays.default
          #self.overlays.tools
        ];
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          targets = [ "wasm32-unknown-unknown" ];
        };
        craneLib = ((crane.mkLib pkgs).overrideToolchain rustToolchain).overrideScope (
          final: prev: {
            trunk = trunk-local;
            wasm-bindgen-cli = wasm-bindgen-cli-local;
          }
        );

        # Utilities
        cargo-leptos-local = craneLib.buildPackage {
          src = craneLib.cleanCargoSource inputs.cargo-leptos-src;
          strictDeps = true;
          nativeBuildInputs = with pkgs; [
            perl
          ];
          buildInputs = with pkgs; [
            openssl
            pkg-config
            wasm-bindgen-cli-local
          ];
          cargoExtraArgs = "--no-default-features --features no_downloads";
          doCheck = false;
        };

        trunk-local = craneLib.buildPackage {
          src = inputs.trunk-src; # Don't clean source
          strictDeps = true;
          buildInputs = with pkgs; [
            pkg-config
            wasm-bindgen-cli-local
          ];
          cargoExtraArgs = "--no-default-features --features rustls";
          doCheck = false;
        };

        # Pinned to match {demo,ssr}/Cargo.toml
        wasm-bindgen-cli-local = pkgs.wasm-bindgen-cli_0_2_100;

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
            trunkExtraBuildArgs = "--config=./demo/Trunk.toml";
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
          packages = [
            cargo-leptos-local
            trunk-local
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
