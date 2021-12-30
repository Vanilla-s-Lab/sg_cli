{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell {
  nativeBuildInputs = [ pkgs.cargo ];
  shellHook = "exec cargo build --release";
  NIX_ENFORCE_PURITY = 0;
}
