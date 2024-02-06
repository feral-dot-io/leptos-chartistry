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
          trunkExtraBuildArgs =
            "--features build-demo --public-url /leptos-chartistry";
          strictDeps = true;
          src = lib.cleanSourceWith {
            src = ./.;
            filter = path: type:
              (lib.hasSuffix ".html" path) || (lib.hasInfix "/public/" path)
              || (craneLib.filterCargoSources path type);
          };
        };

        demo = craneLib.buildTrunkPackage (demoArgs // {
          pname = "chartistry-demo";
          cargoArtifacts =
            craneLib.buildDepsOnly (demoArgs // { doCheck = false; });
          wasm-bindgen-cli = pkgs.wasm-bindgen-cli.override {
            version = "0.2.90";
            hash = "sha256-X8+DVX7dmKh7BgXqP7Fp0smhup5OO8eWEhn26ODYbkQ=";
            cargoHash = "sha256-ckJxAR20GuVGstzXzIj1M0WBFj5eJjrO2/DRMUK5dwM=";
          };
        });
      in {
        devShells.default =
          pkgs.mkShell { packages = with pkgs; [ trunk wasm-bindgen-cli ]; };
        checks.demo = demo;
        packages.demo = demo;
      });
}
