{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [ cargo-tarpaulin cargo clang ];

  # https://github.com/xd009642/tarpaulin#travis-ci-and-coverage-sites
  shellHook = "exec cargo tarpaulin --out Xml";

  # https://nixos.wiki/wiki/Development_environment_with_nix-shell#Troubleshooting
  NIX_ENFORCE_PURITY = 0;
}
