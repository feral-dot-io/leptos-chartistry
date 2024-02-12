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
  };

  outputs = { self, nixpkgs, utils, rust-overlay, crane }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs =
          nixpkgs.legacyPackages.${system}.extend rust-overlay.overlays.default;
        rustToolchain = pkgs.rust-bin.nightly.latest.default.override {
          targets = [ "wasm32-unknown-unknown" ];
        };
        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        demoArgs = with pkgs; {
          CARGO_BUILD_TARGET = "wasm32-unknown-unknown";
          CARGO_PROFILE = "release";
          cargoExtraArgs = "--package=demo";
          trunkExtraBuildArgs = "--public-url /leptos-chartistry";

          pname = "chartistry-demo";
          version = "0.0.1";
          strictDeps = true;
          src = lib.cleanSourceWith {
            src = ./.;
            filter = path: type:
              (lib.hasSuffix ".html" path) || (lib.hasInfix "/assets/" path)
              || (craneLib.filterCargoSources path type);
          };
        };

        cargoArtifacts =
          craneLib.buildDepsOnly (demoArgs // { doCheck = false; });

        demo = craneLib.buildTrunkPackage (demoArgs // {
          inherit cargoArtifacts;
          trunkIndexPath = "demo/index.html";
          wasm-bindgen-cli = pkgs.wasm-bindgen-cli.override {
            version = "0.2.90";
            hash = "sha256-X8+DVX7dmKh7BgXqP7Fp0smhup5OO8eWEhn26ODYbkQ=";
            cargoHash = "sha256-ckJxAR20GuVGstzXzIj1M0WBFj5eJjrO2/DRMUK5dwM=";
          };

          postInstall = ''
            ln -s index.html $out/examples.html
          '';
        });
      in {
        devShells.default =
          pkgs.mkShell { packages = with pkgs; [ trunk wasm-bindgen-cli ]; };
        checks = {
          inherit demo;
          # Check formatting and clippy 
          fmt = craneLib.cargoFmt demoArgs;
          clippy = craneLib.cargoClippy (demoArgs // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          });
        };
        packages.demo = demo;
      });
}
