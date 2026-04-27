{ lib
, stdenv
, rustPlatform
, darwin
, installShellFiles
, cargo-bundle
}:

rustPlatform.buildRustPackage {
  pname = "asteride";
  version = "0.1.5";

  src = ../.;

  cargoLock = {
    lockFile = ../Cargo.lock;
  };

  nativeBuildInputs = [ cargo-bundle ] ++ lib.optionals stdenv.isDarwin [
    installShellFiles
  ];

  postInstall = lib.optionalString stdenv.isDarwin ''
    cd crates/editor
    cargo bundle --release
    cd ../..
    
    mkdir -p $out/Applications
    cp -r target/release/bundle/osx/AsterIDE.app $out/Applications/
    
    xattr -rc $out/Applications/AsterIDE.app 2>/dev/null || true
    
    mkdir -p $out/bin
    rm $out/bin/asteride
    ln -sf $out/Applications/AsterIDE.app/Contents/MacOS/asteride $out/bin/asteride
  '';

  meta = with lib; {
    description = "A Simple Text Editor written in Rust.";
    homepage = "https://github.com/playfairs/AsterIDE";
    license = licenses.gpl3;
    maintainers = [];
    platforms = platforms.darwin;
  };
}
