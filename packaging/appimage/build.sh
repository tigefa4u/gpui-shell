#!/usr/bin/env bash
set -euo pipefail
binary=${1:?Usage: build.sh PATH_TO_BINARY OUTPUT.AppImage}
output=${2:?Usage: build.sh PATH_TO_BINARY OUTPUT.AppImage}
[[ $(uname -m) == x86_64 ]]
assets=$(cd -- "$(dirname -- "$0")" && pwd)
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

appdir="$work/GPUI-Shell.AppDir"
install -Dm755 "$binary" "$appdir/usr/bin/gpui-shell"
install -m755 "$assets/AppRun" "$appdir/AppRun"
install -m644 "$assets/gpui-shell.desktop" "$assets/gpui-shell.svg" "$appdir/"
ln -s gpui-shell.svg "$appdir/.DirIcon"

download() {
    curl --fail --silent --show-error --location --retry 3 "$1" -o "$2"
    printf '%s  %s
' "$3" "$2" | sha256sum --check --status
}
download https://github.com/AppImage/appimagetool/releases/download/1.9.1/appimagetool-x86_64.AppImage \
    "$work/appimagetool" ed4ce84f0d9caff66f50bcca6ff6f35aae54ce8135408b3fa33abfc3cb384eb0
download https://github.com/AppImage/type2-runtime/releases/download/20251108/runtime-x86_64 \
    "$work/runtime" 2fca8b443c92510f1483a883f60061ad09b46b978b2631c807cd873a47ec260d
chmod +x "$work/appimagetool"
mkdir -p -- "$(dirname -- "$output")"
ARCH=x86_64 "$work/appimagetool" --appimage-extract-and-run \
    --runtime-file "$work/runtime" "$appdir" "$output"
chmod +x -- "$output"
