let
  sources = import ./nix/sources.nix;
  moz_overlay = import sources.nixpkgs-mozilla;
  pkgs = import sources.nixpkgs { overlays = [moz_overlay]; };
in
pkgs.mkShell {
  buildInputs = [
    pkgs.latest.rustChannels.nightly.rust
  ];
  RUST_BACKTRACE = 1;
}