{
  description = "Development environment for AB Macros";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        
        # Select the rust toolchain to use
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          # Include WASM target for frontend
          targets = [ "wasm32-unknown-unknown" ];
          extensions = [ "rust-src" "rust-analyzer" ];
        };

        # Native dependencies needed for the project
        nativeBuildInputs = with pkgs; [
          pkg-config
          rustToolchain
        ];

        # Libraries needed for the project
        buildInputs = with pkgs; [
          # System libraries
          openssl
          openssl.dev
          libiconv
          
          # For Sled database
          zstd
          zlib
          
          # For Twilio and networking
          curl

          # Development tools
          trunk
          wasm-pack
          wasm-bindgen-cli
          nodePackages.tailwindcss
        ];

        # Development shell
        devShell = pkgs.mkShell {
          inherit buildInputs nativeBuildInputs;
          
          # Environment variables
          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
          RUST_BACKTRACE = 1;
          
          # Needed for macOS
          APPEND_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
          
          shellHook = ''
            export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath buildInputs}:$LD_LIBRARY_PATH"
            export LIBRARY_PATH="$APPEND_LIBRARY_PATH:$LIBRARY_PATH"
            
            echo "AB-Macros development environment loaded!"
            echo "Run 'cargo build' to build the project."
          '';
        };
      in
      {
        devShells.default = devShell;
        
        # Add a formatter for the flake.nix file
        formatter = pkgs.nixpkgs-fmt;
      }
    );
}
