set windows-shell := ["powershell.exe", "-NoProfile", "-Command"]
build_dir := "build"

build-nix:
    nix build

clean:
    rm -rf build
    rm -f result

create-mac-dmg:
    @just build-nix
    rm -rf "./{{ build_dir }}"
    mkdir -p "./{{ build_dir }}"
    cp -r ./result/* "./{{ build_dir }}"
    chmod -R u+w "./{{ build_dir }}"

    create-dmg \
      --volname "AsterIDE" \
      --window-size 500 300 \
      --icon-size 96 \
      --icon "AsterIDE.app" 125 150 \
      --app-drop-link 375 150 \
      "./{{ build_dir }}/AsterIDE.dmg" \
      "./{{ build_dir }}/Applications/AsterIDE.app"
    mv "./{{ build_dir }}/AsterIDE.dmg" ./AsterIDE.dmg
    rm -rf "./{{ build_dir }}"
    mkdir -p "./{{ build_dir }}"
    mv ./AsterIDE.dmg "./{{ build_dir }}/AsterIDE.dmg"
