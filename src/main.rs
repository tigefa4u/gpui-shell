use anyhow::{Context as _, Result};
use gpui::{
    AppContext, Application, Bounds, Context, Edges, Entity, IntoElement, ParentElement, Render,
    Styled, Window, WindowBounds, WindowOptions, div, px, rgb, size,
};
use gpui_terminal::{ColorPalette, TerminalConfig, TerminalView};
use portable_pty::{Child, CommandBuilder, PtySize, native_pty_system};

// Keep the child associated with this window. On a window close, stop it and reap it.
struct ShellChild(Box<dyn Child + Send + Sync>);

impl Drop for ShellChild {
    fn drop(&mut self) {
        if matches!(self.0.try_wait(), Ok(None)) && self.0.kill().is_ok() {
            let _ = self.0.wait();
        }
    }
}

struct ShellWindow {
    terminal: Entity<TerminalView>,
    shell_name: String,
    _child: ShellChild,
}

impl Render for ShellWindow {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .overflow_hidden()
            .bg(rgb(0x11161e))
            .child(
                div()
                    .h(px(36.0))
                    .flex_none()
                    .px_4()
                    .flex()
                    .items_center()
                    .justify_between()
                    .border_b_1()
                    .border_color(rgb(0x2b3746))
                    .text_sm()
                    .text_color(rgb(0xe2eaf4))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(div().size(px(7.0)).rounded_full().bg(rgb(0x6fe3bc)))
                            .child(self.shell_name.clone()),
                    )
                    .child(div().text_color(rgb(0xa3b4ca)).child("Local shell")),
            )
            .child(
                div()
                    .flex_1()
                    .min_h_0()
                    .overflow_hidden()
                    .child(self.terminal.clone()),
            )
    }
}

fn pty_size(cols: usize, rows: usize) -> PtySize {
    PtySize {
        cols: cols.clamp(1, u16::MAX as usize) as u16,
        rows: rows.clamp(1, u16::MAX as usize) as u16,
        pixel_width: 0,
        pixel_height: 0,
    }
}

fn main() {
    Application::new().run(|cx| {
        if let Err(error) = open_terminal(cx) {
            eprintln!("Could not open GPUI Shell: {error:#}");
            cx.quit();
            std::process::exit(1);
        }
    });
}

fn open_terminal(cx: &mut gpui::App) -> Result<()> {
    let shell = std::env::var_os("SHELL")
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| {
            if cfg!(windows) {
                std::env::var_os("COMSPEC").unwrap_or_else(|| "cmd.exe".into())
            } else {
                "/bin/sh".into()
            }
        });
    let shell_name = std::path::Path::new(&shell)
        .file_name()
        .unwrap_or(&shell)
        .to_string_lossy()
        .into_owned();
    let config = TerminalConfig {
        font_family: if cfg!(target_os = "macos") {
            "Menlo"
        } else if cfg!(windows) {
            "Consolas"
        } else {
            "DejaVu Sans Mono"
        }
        .into(),
        font_size: px(15.0),
        padding: Edges::all(px(14.0)),
        colors: ColorPalette::builder()
            .background(0x0d, 0x11, 0x17)
            .foreground(0xe2, 0xea, 0xf4)
            .cursor(0x6f, 0xe3, 0xbc)
            .build(),
        ..TerminalConfig::default()
    };
    let pair = native_pty_system()
        .openpty(pty_size(config.cols, config.rows))
        .context("open pseudo-terminal")?;
    let reader = pair.master.try_clone_reader().context("open PTY reader")?;
    let writer = pair.master.take_writer().context("open PTY writer")?;
    let mut command = CommandBuilder::new(shell);
    command.cwd(std::env::current_dir().context("read launch directory")?);
    command.env("TERM", "xterm-256color");
    command.env("COLORTERM", "truecolor");
    let child = ShellChild(pair.slave.spawn_command(command).context("start shell")?);
    drop(pair.slave);
    let master = std::sync::Mutex::new(pair.master);

    let bounds = Bounds::centered(None, size(px(900.0), px(540.0)), cx);
    cx.open_window(
        WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            window_min_size: Some(size(px(320.0), px(180.0))),
            titlebar: Some(gpui::TitlebarOptions {
                title: Some("GPUI Shell".into()),
                ..Default::default()
            }),
            ..Default::default()
        },
        move |window, cx| {
            let terminal = cx.new(|cx| {
                TerminalView::new(writer, reader, config, cx)
                    .with_resize_callback(move |cols, rows| {
                        if let Err(error) = master
                            .lock()
                            .expect("PTY lock poisoned")
                            .resize(pty_size(cols, rows))
                        {
                            eprintln!("PTY resize failed: {error}");
                        }
                    })
                    .with_exit_callback(|_, cx| cx.quit())
            });
            terminal.read(cx).focus_handle().focus(window);
            cx.new(|_| ShellWindow {
                terminal,
                shell_name,
                _child: child,
            })
        },
    )?;
    cx.on_window_closed(|cx| {
        if cx.windows().is_empty() {
            cx.quit();
        }
    })
    .detach();
    cx.activate(true);
    Ok(())
}
