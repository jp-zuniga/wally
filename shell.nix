{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  packages = [
    pkgs.bacon
    pkgs.cargo
    pkgs.clippy
    pkgs.rustc
    pkgs.rustfmt
  ];
}
