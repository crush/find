use anyhow::Result;
use inquire::Select;

pub fn select(paths: &[String]) -> Result<String> {
    let display: Vec<&str> = paths.iter().map(String::as_str).collect();

    let selection = Select::new("select directory:", display)
        .with_page_size(10)
        .prompt()?;

    Ok(selection.to_string())
}
