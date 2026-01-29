use crossterm::cursor::{Hide, MoveToColumn, MoveUp, Show};
use crossterm::event::{self, Event, KeyCode, KeyEventKind, MouseEventKind};
use crossterm::terminal::{self, Clear, ClearType};
use crossterm::QueueableCommand;
use std::io::{stderr, Write};

const PAGE_SIZE: usize = 5;

pub fn select(items: &[(String, u32)]) -> Option<String> {
    interactive(items, false)
}

pub fn browse(items: &[(String, u32)]) -> Option<String> {
    interactive(items, true)
}

fn interactive(items: &[(String, u32)], show_score_default: bool) -> Option<String> {
    let mut selected = 0;
    let mut offset = 0;
    let mut show_score = show_score_default;
    let total = items.len();
    let visible = total.min(PAGE_SIZE);
    let mut err = stderr();

    let _ = terminal::enable_raw_mode();
    let _ = crossterm::execute!(err, crossterm::event::EnableMouseCapture);
    let _ = err.queue(Hide);
    let _ = err.flush();

    print_page(&mut err, items, selected, offset, visible, show_score);

    let result = loop {
        if let Ok(ev) = event::read() {
            match ev {
                Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Enter => break Some(items[selected].0.clone()),
                    KeyCode::Esc | KeyCode::Char('q') => break None,
                    KeyCode::Char('c')
                        if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) =>
                    {
                        break None
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        if selected > 0 {
                            selected -= 1;
                            if selected < offset {
                                offset = selected;
                            }
                        }
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        if selected < total - 1 {
                            selected += 1;
                            if selected >= offset + visible {
                                offset = selected - visible + 1;
                            }
                        }
                    }
                    KeyCode::Tab => {
                        show_score = !show_score;
                    }
                    _ => continue,
                },
                Event::Mouse(mouse) => match mouse.kind {
                    MouseEventKind::ScrollUp => {
                        if selected > 0 {
                            selected -= 1;
                            if selected < offset {
                                offset = selected;
                            }
                        }
                    }
                    MouseEventKind::ScrollDown => {
                        if selected < total - 1 {
                            selected += 1;
                            if selected >= offset + visible {
                                offset = selected - visible + 1;
                            }
                        }
                    }
                    _ => continue,
                },
                _ => continue,
            }
            clear_page(&mut err, visible);
            print_page(&mut err, items, selected, offset, visible, show_score);
        }
    };

    clear_page(&mut err, visible);
    let _ = crossterm::execute!(err, crossterm::event::DisableMouseCapture);
    let _ = err.queue(Show);
    let _ = err.flush();
    let _ = terminal::disable_raw_mode();
    result
}

fn print_page(
    err: &mut std::io::Stderr,
    items: &[(String, u32)],
    selected: usize,
    offset: usize,
    visible: usize,
    show_score: bool,
) {
    let max_score = items.iter().map(|(_, s)| *s).max().unwrap_or(1);
    let end = (offset + visible).min(items.len());

    for (i, (path, score)) in items[offset..end].iter().enumerate() {
        let idx = offset + i;
        let name = path.rsplit('/').next().unwrap_or(path);
        let prefix = if idx == selected { "> " } else { "  " };

        if show_score {
            let bar_width = (*score as f64 / max_score as f64 * 8.0) as usize;
            let bar: String = (0..8).map(|j| if j < bar_width { '=' } else { ' ' }).collect();
            let _ = write!(err, "{}{}\x1b[90m {:>4} {}\x1b[0m", prefix, name, score, bar);
        } else {
            let _ = write!(err, "{}{}", prefix, name);
        }

        if i < visible - 1 && idx < items.len() - 1 {
            let _ = write!(err, "\r\n");
        }
    }
    let _ = err.flush();
}

fn clear_page(err: &mut std::io::Stderr, visible: usize) {
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
