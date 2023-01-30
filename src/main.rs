use inquire::{
    ui::{Attributes, Color, RenderConfig, StyleSheet},
    Select,
};
use color_eyre::eyre::Result;

pub mod network;
pub mod utils;

fn get_render_cfg() -> RenderConfig {
    RenderConfig {
        answer: StyleSheet::new()
            .with_attr(Attributes::ITALIC)
            .with_fg(Color::LightCyan),
        help_message: StyleSheet::new()
            .with_fg(Color::LightCyan),
        ..Default::default()
    }
}

fn menu(items: &[String]) -> String {
    Select::new("What would you like to do?", items.to_vec())
        .with_vim_mode(false)
        .with_help_message("Vim mode is disabled")
        .prompt()
        .unwrap_or_else(|e| e.to_string())
}

fn main() -> Result<()> {
    inquire::set_global_render_config(get_render_cfg());

    loop {
        match menu(&["SSH".into(), "Ping Server".into(), "Exit".into()]).as_str() {
            "SSH" => {
                println!("SSHing to the server...");
                network::ssh_into();
            },
            "Ping Server" => {
                network::ping();
            },
            "Exit" => break,
            err => {
                println!("{err}");
                std::process::exit(1)
            }
        }
    }

    Ok(())
}
