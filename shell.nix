{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell {
    # nativeBuildInputs is usually what you want -- tools you need to run
    nativeBuildInputs = with pkgs.buildPackages; [ rustc cargo gcc clang llvmPackages.clang llvmPackages.libclang rustfmt clippy rust-analyzer ];

    RUST_SRC_PATH="${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
    LIBCLANG_PATH="/nix/store/xibf2ayyaljj1r7wgjf4n7n4vg5z8n2v-clang-21.1.8-lib/lib";
}
