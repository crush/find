use crossterm::cursor::{Hide, MoveToColumn, MoveUp, Show};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{self, Clear, ClearType};
use crossterm::QueueableCommand;
use std::io::{stderr, Write};

pub fn select(paths: &[String]) -> Option<String> {
    let mut selected = 0;
    let mut show_path = false;
    let len = paths.len();
    let mut err = stderr();

    let _ = terminal::enable_raw_mode();
    let _ = err.queue(Hide);
    let _ = err.flush();

    print_list(&mut err, paths, selected, show_path);

    let result = loop {
        if let Ok(Event::Key(key)) = event::read() {
            if key.kind != KeyEventKind::Press {
                continue;
            }
            match key.code {
                KeyCode::Enter => break Some(paths[selected].clone()),
                KeyCode::Esc | KeyCode::Char('q') => break None,
                KeyCode::Char('c') if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) => break None,
                KeyCode::Up | KeyCode::Char('k') => {
                    selected = selected.saturating_sub(1);
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    selected = (selected + 1).min(len - 1);
                }
                KeyCode::Tab => {
                    show_path = !show_path;
                }
                _ => continue,
            }
            clear_list(&mut err, len);
            print_list(&mut err, paths, selected, show_path);
        }
    };

    clear_list(&mut err, len);
    let _ = err.queue(Show);
    let _ = err.flush();
    let _ = terminal::disable_raw_mode();
    result
}

fn print_list(err: &mut std::io::Stderr, paths: &[String], selected: usize, show_path: bool) {
    for (i, path) in paths.iter().enumerate() {
        let name = path.rsplit('/').next().unwrap_or(path);
        let prefix = if i == selected { "> " } else { "  " };
        if show_path {
            let _ = write!(err, "{}{}\x1b[90m  {}\x1b[0m", prefix, name, path);
        } else {
            let _ = write!(err, "{}{}", prefix, name);
        }
        if i < paths.len() - 1 {
            let _ = write!(err, "\r\n");
        }
    }
    let _ = err.flush();
}

fn clear_list(err: &mut std::io::Stderr, len: usize) {
    if len > 1 {
        let _ = err.queue(MoveUp((len - 1) as u16));
    }
    let _ = err.queue(MoveToColumn(0));
    let _ = err.flush();
    for i in 0..len {
        let _ = err.queue(Clear(ClearType::CurrentLine));
        if i < len - 1 {
            let _ = write!(err, "\r\n");
        }
    }
    if len > 1 {
        let _ = err.queue(MoveUp((len - 1) as u16));
    }
    let _ = err.queue(MoveToColumn(0));
    let _ = err.flush();
}
