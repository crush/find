use crossterm::cursor::{Hide, MoveToColumn, MoveUp, Show};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::style::Print;
use crossterm::terminal::{self, Clear, ClearType};
use crossterm::ExecutableCommand;
use std::io::{stdout, Write};

pub fn select(paths: &[String]) -> Option<String> {
    let mut selected = 0;
    let mut show_path = false;
    let len = paths.len();

    terminal::enable_raw_mode().ok()?;
    stdout().execute(Hide).ok()?;

    draw(paths, selected, show_path);

    loop {
        if let Event::Key(key) = event::read().ok()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }
            match key.code {
                KeyCode::Enter => {
                    cleanup(len);
                    return Some(paths[selected].clone());
                }
                KeyCode::Esc | KeyCode::Char('q') => {
                    cleanup(len);
                    return None;
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    selected = selected.saturating_sub(1);
                    redraw(paths, selected, show_path, len);
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    selected = (selected + 1).min(len - 1);
                    redraw(paths, selected, show_path, len);
                }
                KeyCode::Tab => {
                    show_path = !show_path;
                    redraw(paths, selected, show_path, len);
                }
                _ => {}
            }
        }
    }
}

fn draw(paths: &[String], selected: usize, show_path: bool) {
    let mut out = stdout();
    for (i, path) in paths.iter().enumerate() {
        let name = path.rsplit('/').next().unwrap_or(path);
        let prefix = if i == selected { "> " } else { "  " };
        if show_path {
            let _ = out.execute(Print(format!("{}{}\x1b[90m  {}\x1b[0m\n", prefix, name, path)));
        } else {
            let _ = out.execute(Print(format!("{}{}\n", prefix, name)));
        }
    }
    let _ = out.flush();
}

fn redraw(paths: &[String], selected: usize, show_path: bool, len: usize) {
    let mut out = stdout();
    let _ = out.execute(MoveUp(len as u16));
    let _ = out.execute(MoveToColumn(0));
    for _ in 0..len {
        let _ = out.execute(Clear(ClearType::CurrentLine));
        let _ = out.execute(Print("\n"));
    }
    let _ = out.execute(MoveUp(len as u16));
    draw(paths, selected, show_path);
}

fn cleanup(len: usize) {
    let mut out = stdout();
    let _ = out.execute(MoveUp(len as u16));
    let _ = out.execute(MoveToColumn(0));
    for _ in 0..len {
        let _ = out.execute(Clear(ClearType::CurrentLine));
        let _ = out.execute(Print("\n"));
    }
    let _ = out.execute(MoveUp(len as u16));
    let _ = out.execute(Clear(ClearType::CurrentLine));
    let _ = out.execute(Show);
    let _ = terminal::disable_raw_mode();
}
