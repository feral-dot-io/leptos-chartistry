{
  description = "Chartistry is a charting library for Leptos";
  inputs = {
    nixpkgs.url = "nixpkgs";
    utils.url = "flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, utils, rust-overlay, crane, advisory-db }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs =
          nixpkgs.legacyPackages.${system}.extend rust-overlay.overlays.default;
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          targets = [ "wasm32-unknown-unknown" ];
        };
        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        src = with pkgs;
          lib.cleanSourceWith {
            src = ./.;
            filter = path: type:
              (lib.hasSuffix ".html" path) || (lib.hasInfix "/assets/" path)
              || (craneLib.filterCargoSources path type);
          };

        commonArgs = {
          inherit src;
          pname = "leptos-chartistry-workspace";
          version = "0.0.1";
          CARGO_BUILD_TARGET = "wasm32-unknown-unknown";
          CARGO_PROFILE = "release";
        };

        cargoArtifacts =
          craneLib.buildDepsOnly (commonArgs // { doCheck = false; });

        demo = craneLib.buildTrunkPackage (commonArgs // {
          inherit cargoArtifacts;
          pname = "chartistry-demo";
          version = "0.0.1";
          strictDeps = true;

          cargoExtraArgs = "--package=demo";
          trunkExtraBuildArgs = "--public-url /leptos-chartistry";
          trunkIndexPath = "demo/index.html";
          # Create symlinks for each of our pages. Enables a static site.
          postInstall = ''
            ln -s index.html $out/examples.html
          '';

          wasm-bindgen-cli = pkgs.wasm-bindgen-cli.override {
            version = "0.2.90";
            hash = "sha256-X8+DVX7dmKh7BgXqP7Fp0smhup5OO8eWEhn26ODYbkQ=";
            cargoHash = "sha256-ckJxAR20GuVGstzXzIj1M0WBFj5eJjrO2/DRMUK5dwM=";
          };
        });

      in {
        devShells.default =
          pkgs.mkShell { packages = with pkgs; [ trunk wasm-bindgen-cli ]; };
        checks = {
          inherit demo;

          clippy = craneLib.cargoClippy (commonArgs // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          });

          doc = craneLib.cargoDoc (commonArgs // {
            inherit cargoArtifacts;
            cargoDocExtraArgs = "--workspace";
          });

          audit = craneLib.cargoAudit { inherit src advisory-db; };
          fmt = craneLib.cargoFmt commonArgs;
        };
        packages.demo = demo;
      });
}
