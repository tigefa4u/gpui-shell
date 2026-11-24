# GPUI Shell

A single-window desktop terminal written in Rust. GPUI renders the native window and its root view. [gpui-terminal](https://docs.rs/gpui-terminal/0.1.0/gpui_terminal/) provides the Alacritty-backed terminal grid, ANSI colors, keyboard input, and scrollback. [portable-pty](https://docs.rs/portable-pty/0.9.0/portable_pty/) connects that view to an interactive local shell.

## Run from source

Install Rust stable with Cargo and run this from the extracted project folder:

```sh
cargo run --locked
```

On Ubuntu/Debian, install the desktop build prerequisites first:

```sh
sudo apt install build-essential pkg-config libfontconfig1-dev libxkbcommon-dev \
  libxkbcommon-x11-dev libwayland-dev libxcb1-dev libxcb-render0-dev \
  libxcb-shape0-dev libxcb-xfixes0-dev libx11-xcb-dev libvulkan1 \
  mesa-vulkan-drivers fonts-dejavu-core fonts-dejavu-mono
```

Run in a desktop session with GPUI-compatible graphics support (X11 or Wayland on Linux). On macOS, install Xcode Command Line Tools with `xcode-select --install` if you don't already have a C toolchain. On Windows, use the Rust MSVC toolchain and Visual Studio's Desktop development with C++ workload.

The app opens `$SHELL` in the launch directory, falling back to `/bin/sh` on Unix or `%COMSPEC%` / `cmd.exe` on Windows. Type commands directly into the focused terminal. The window resizes the pseudo-terminal, the mouse wheel scrolls through history, and `exit` ends the session. Like a normal terminal, commands run with your local user privileges.

## Try it

- Run `pwd` and check that it shows the launch directory (or `cd` on Windows).
- On Unix, run `stty size`, resize the window, and run it again. The reported rows and columns should change.
- On Unix, run `printf '\033[32mgreen text\033[0m\n'` to check ANSI colors.
- Run `exit` to close the app.

The application follows the window-and-root-view pattern in [GPUI’s Start Here examples](https://gpui.rs/examples/#start-here).

Verified on Ubuntu 24.04 (X11 via Xvfb with Mesa software Vulkan). macOS, Windows, and Wayland were not tested.

The bundled xattr 0.2.3 patch keeps the published GPUI dependency tree compatible with current libc on Linux and Android; its upstream licenses are in vendor/xattr.
