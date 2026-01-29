use anyhow::Result;
use inquire::ui::{Color, IndexPrefix, RenderConfig, StyleSheet, Styled};
use inquire::Select;

fn format_paths(paths: &[String]) -> Vec<String> {
    paths
        .iter()
        .map(|path| path.rsplit('/').next().unwrap_or(path).to_string())
        .collect()
}

fn config() -> RenderConfig<'static> {
    RenderConfig {
        prompt: StyleSheet::new(),
        highlighted_option_prefix: Styled::new(">").with_fg(Color::LightCyan),
        option_index_prefix: IndexPrefix::None,
        scroll_up_prefix: Styled::new(""),
        scroll_down_prefix: Styled::new(""),
        ..RenderConfig::empty()
    }
}

pub fn select(paths: &[String]) -> Result<String> {
    let display = format_paths(paths);
    let display_refs: Vec<&str> = display.iter().map(|s| s.as_str()).collect();

    let idx = Select::new("", display_refs)
        .with_page_size(10)
        .with_render_config(config())
        .without_help_message()
        .prompt()?;

    let selected_idx = display.iter().position(|d| d == idx).unwrap_or(0);
    Ok(paths[selected_idx].clone())
}
