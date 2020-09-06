let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
in
with nixpkgs;
stdenv.mkDerivation {
  name = "moz_overlay_shell";
  buildInputs = [
    # to use the latest nightly:
    nixpkgs.latest.rustChannels.nightly.rust
   ];
  RUST_BACKTRACE = 1;
}
