use clap::Parser;
use color_eyre::{Result, eyre::ContextCompat};
use colored::Colorize;
use dialoguer::{Confirm, theme::ColorfulTheme};
use inkjet::{
    Highlighter, Language,
    formatter::Terminal,
    theme::{Theme, vendored},
};
use termcolor::{ColorChoice, StandardStream};

mod editor_plugins;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The API key to use
    #[arg(short, long)]
    key: String,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    println!(
        "{} {}\n",
        "Welcome to Hackatime,".bold().blue(),
        "Mahad!".bold()
    );

    println!(
        "Here's the {} file I'm planning to write:\n",
        "~/.wakatime.cfg".green()
    );

    let generated_config: String =
        include_str!("./config.ini.template").replace("{{key}}", &cli.key);
    print_ini(&generated_config)?;
    println!();

    let write = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Should I write this to your WakaTime config?")
        .interact()
        .unwrap();

    if !write {
        eprintln!("{}", "Understood, exiting now.".dimmed());
        return Ok(());
    }

    let config_path = dirs::home_dir()
        .wrap_err("Could not find home directory")?
        .join(".wakatime.cfg");

    std::fs::write(&config_path, &generated_config)?;
    println!(
        "{} {}",
        "✔".green().bold(),
        format!("Config written to {}", config_path.display()).green()
    );

    let editors = editor_plugins::all_editors();
    for editor in editors {
        dbg!(editor.name());
    }

    Ok(())
}

fn print_ini(ini: &str) -> Result<()> {
    let mut highlighter = Highlighter::new();
    let theme = Theme::from_helix(vendored::AYU_DARK)?;

    let lines: Vec<&str> = ini.lines().collect();
    let max_width = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0);
    let box_width = max_width + 2;

    println!("┌{}┐", "─".repeat(box_width));

    for line in lines {
        print!("│ ");
        let stream = StandardStream::stdout(ColorChoice::Always);
        let formatter = Terminal::new(theme.clone(), stream);
        highlighter.highlight_to_writer(Language::Ini, &formatter, line, &mut std::io::sink())?;
        let padding = max_width - line.chars().count();
        println!("{} │", " ".repeat(padding));
    }

    println!("└{}┘", "─".repeat(box_width));

    Ok(())
}
