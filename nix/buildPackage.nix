{ lib
, stdenv
, rustPlatform
, darwin
, installShellFiles
}:

rustPlatform.buildRustPackage {
  pname = "asteride";
  version = "0.1.0";

  src = ../.;

  cargoLock = {
    lockFile = ../Cargo.lock;
  };

  nativeBuildInputs = lib.optionals stdenv.isDarwin [
    installShellFiles
  ];

  buildInputs = lib.optionals stdenv.isDarwin (with darwin.apple_sdk.frameworks; [
    AppKit
    CoreGraphics
    CoreServices
    Foundation
    IOKit
  ]);

  postInstall = lib.optionalString stdenv.isDarwin ''
    cargo install cargo-bundle --root $TMPDIR/cargo-bundle
    export PATH="$TMPDIR/cargo-bundle/bin:$PATH"
    
    cd crates/editor
    cargo bundle --release
    cd ../..
    
    mkdir -p $out/Applications
    cp -r target/release/bundle/osx/AsterIDE.app $out/Applications/
    
    mkdir -p $out/bin
    ln -s $out/Applications/AsterIDE.app/Contents/MacOS/asteride $out/bin/asteride
  '';

  meta = with lib; {
    description = "A Simple Text Editor written in Rust.";
    homepage = "https://github.com/playfairs/AsterIDE";
    license = licenses.gpl3;
    maintainers = [];
    platforms = platforms.darwin;
  };
}
