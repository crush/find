use crossterm::cursor::{Hide, MoveToColumn, MoveUp, Show};
use crossterm::event::{self, Event, KeyCode, KeyEventKind, MouseEventKind, EnableMouseCapture, DisableMouseCapture};
use crossterm::terminal::{self, Clear, ClearType};
use crossterm::QueueableCommand;
use std::io::{stderr, Write};

const VISIBLE: usize = 10;

pub fn select(paths: &[String]) -> Option<String> {
    let mut show_path = false;
    let len = paths.len();
    let mut err = stderr();

    let _ = terminal::enable_raw_mode();
    let _ = err.queue(Hide);
    let _ = err.queue(EnableMouseCapture);
    let _ = err.flush();

    let visible = len.min(VISIBLE);
    print_list(&mut err, paths, visible, show_path);

    let result = loop {
        if let Ok(event) = event::read() {
            match event {
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    match key.code {
                        KeyCode::Esc | KeyCode::Char('q') => break None,
                        KeyCode::Char('c') if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) => break None,
                        KeyCode::Char(c @ '1'..='9') => {
                            let idx = (c as usize) - ('1' as usize);
                            if idx < len {
                                break Some(paths[idx].clone());
                            }
                        }
                        KeyCode::Char('0') => {
                            if len >= 10 {
                                break Some(paths[9].clone());
                            }
                        }
                        KeyCode::Tab => {
                            show_path = !show_path;
                            clear_list(&mut err, visible);
                            print_list(&mut err, paths, visible, show_path);
                        }
                        _ => continue,
                    }
                }
                Event::Mouse(mouse) => {
                    match mouse.kind {
                        MouseEventKind::ScrollUp | MouseEventKind::ScrollDown => continue,
                        _ => continue,
                    }
                }
                _ => continue,
            }
        }
    };

    clear_list(&mut err, visible);
    let _ = err.queue(DisableMouseCapture);
    let _ = err.queue(Show);
    let _ = err.flush();
    let _ = terminal::disable_raw_mode();
    result
}

fn print_list(err: &mut std::io::Stderr, paths: &[String], visible: usize, show_path: bool) {
    for i in 0..visible {
        if i >= paths.len() {
            break;
        }
        let path = &paths[i];
        let name = path.rsplit('/').next().unwrap_or(path);
        let num = if i == 9 { 0 } else { i + 1 };
        if show_path {
            let _ = write!(err, "\x1b[90m{}\x1b[0m {}\x1b[90m  {}\x1b[0m", num, name, path);
        } else {
            let _ = write!(err, "\x1b[90m{}\x1b[0m {}", num, name);
        }
        if i < visible - 1 {
            let _ = write!(err, "\r\n");
        }
    }
    let _ = err.flush();
}

fn clear_list(err: &mut std::io::Stderr, visible: usize) {
    if visible > 1 {
        let _ = err.queue(MoveUp((visible - 1) as u16));
    }
    let _ = err.queue(MoveToColumn(0));
    let _ = err.flush();
    for i in 0..visible {
        let _ = err.queue(Clear(ClearType::CurrentLine));
        if i < visible - 1 {
            let _ = write!(err, "\r\n");
        }
    }
    if visible > 1 {
        let _ = err.queue(MoveUp((visible - 1) as u16));
    }
    let _ = err.queue(MoveToColumn(0));
    let _ = err.flush();
}
