use clap::builder::styling;
use clap::{Arg, ArgAction, Command};
use colored::Colorize;

/// Define CLI arguments and styling
pub fn cli(version: &'static str) -> Command {
    let styles = styling::Styles::styled()
        .header(styling::AnsiColor::White.on_default() | styling::Effects::BOLD)
        .usage(styling::AnsiColor::White.on_default() | styling::Effects::BOLD)
        .literal(styling::AnsiColor::BrightCyan.on_default())
        .placeholder(styling::AnsiColor::Cyan.on_default());

    Command::new("mezzotint")
        .version(version)
        .about(format!("{}{} - {}", "mezzo".bold().underline(), "tint", "is a tool to turn your container into an App Bundle"))
        // Config
        .arg(
            Arg::new("exe")
                .short('x')
                .long("exe")
                .help("Specify path to an executable which needs to be preserved.")
        )
        .arg(
            Arg::new("profile")
                .short('p')
                .long("profile")
                .help("Profile, describing whole setup"),
        )
        // Other
        .arg(
            Arg::new("debug")
                .short('d')
                .long("debug")
                .action(ArgAction::SetTrue)
                .help("Set debug mode for more verbose output."),
        )
        .arg(
            Arg::new("version")
                .short('v')
                .long("version")
                .action(ArgAction::SetTrue)
                .help("Get current version."),
        )
        .disable_version_flag(true)
        .disable_colored_help(false)
        .styles(styles)
}
