use anyhow::Result;
use inquire::ui::{Attributes, RenderConfig, StyleSheet, Styled};
use inquire::Select;

fn short(path: &str) -> String {
    let parts: Vec<&str> = path.split('/').collect();
    if parts.len() <= 3 {
        return path.to_string();
    }
    format!(".../{}", parts[parts.len() - 3..].join("/"))
}

fn config() -> RenderConfig<'static> {
    RenderConfig {
        prompt: StyleSheet::new().with_attr(Attributes::BOLD),
        highlighted_option_prefix: Styled::new(">"),
        scroll_up_prefix: Styled::new("^"),
        scroll_down_prefix: Styled::new("v"),
        ..RenderConfig::default_colored()
    }
}

pub fn select(paths: &[String]) -> Result<String> {
    let display: Vec<String> = paths.iter().map(|p| short(p)).collect();
    let display_refs: Vec<&str> = display.iter().map(|s| s.as_str()).collect();

    let idx = Select::new("", display_refs)
        .with_page_size(8)
        .with_render_config(config())
        .without_help_message()
        .prompt()?;

    let selected_idx = display.iter().position(|d| d == idx).unwrap_or(0);
    Ok(paths[selected_idx].clone())
}
