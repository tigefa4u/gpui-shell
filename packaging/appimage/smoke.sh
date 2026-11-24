#!/usr/bin/env bash
set -euo pipefail
image=$(realpath "${1:?Usage: smoke.sh PATH_TO_APPIMAGE}")
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT
cd "$work"

cat > smoke-shell <<'SHELL'
#!/bin/sh
set -eu
test -t 0 && test -t 1
pwd > shell-cwd
exec /bin/sh
SHELL
chmod +x smoke-shell
# Make the X11 smoke test independent of the runner's Wayland and GPU settings.
unset WAYLAND_DISPLAY WAYLAND_SOCKET
icds=(/usr/share/vulkan/icd.d/lvp_icd*.json)
export VK_DRIVER_FILES="${icds[0]}"
SHELL="$work/smoke-shell" "$image" --appimage-extract-and-run > app.log 2>&1 &
pid=$!
trap 'cat app.log; kill "$pid" 2>/dev/null || true; wait "$pid" 2>/dev/null || true; rm -rf -- "$work"' EXIT
window=$(timeout 20s xdotool search --sync --onlyvisible --name '^GPUI Shell$')
[[ $(cat shell-cwd) == "$work" ]]
xdotool windowfocus --sync "$window"
xdotool type --clearmodifiers 'printf smoke-ok > terminal-output; exit'
xdotool key Return
for ((i=0; i<100; i++)); do
    if ! kill -0 "$pid" 2>/dev/null; then
        wait "$pid"
        [[ $(cat terminal-output) == smoke-ok ]]
        exit 0
    fi
    sleep 0.2
done
echo 'GPUI Shell did not exit after the smoke command.' >&2
exit 1
