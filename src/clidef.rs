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
        .override_usage(format!("{} {} {}", "mezzotint".bright_cyan(), "[OPTIONS]".cyan(), "[FILTERS]".cyan()))
        // Config
        .arg(
            Arg::new("exe")
                .short('x')
                .long("exe")
                .conflicts_with("profile")
                .help("Specify path to an executable which needs to be preserved.")
        )
        .arg(
            Arg::new("profile")
                .short('p')
                .long("profile")
                .conflicts_with("exe")
                .help("Profile, describing whole setup"),
        )

        // Filters
        .next_help_heading("Filters")
        /*
        f_prune: Vec<String>

        packages: Vec<String>
        targets: Vec<String>
         */
        .arg(
            Arg::new("f_l10n").long("l10n").action(clap::ArgAction::SetTrue).help("Filter localisation data")
        )
        .arg(
            Arg::new("f_i18n").long("i18n").action(clap::ArgAction::SetTrue).help("Filter internationalisation data")
        )
        .arg(
            Arg::new("f_doc").long("doc").action(clap::ArgAction::SetTrue).help("Filter documents, texts, licences etc")
        )
        .arg(
            Arg::new("f_man").long("man").action(clap::ArgAction::SetTrue).help("Filter manpages")
        )
        .arg(
            Arg::new("f_dir").long("dirs").action(clap::ArgAction::SetTrue).help("Filter empty directories (except required)")
        )
        .arg(
            Arg::new("f_log").long("logs").action(clap::ArgAction::SetTrue).help("Filter any kind of logs")
        )

        // Other
        .next_help_heading("Other")
        .arg(
            Arg::new("debug")
                .short('d')
                .long("debug")
                .action(ArgAction::SetTrue)
                .help("Set debug mode for more verbose output."),
        )
        .arg(
            Arg::new("help")
                .short('h')
                .long("help")
                .action(ArgAction::SetTrue)
                .help("Display help"),
        )
        .arg(
            Arg::new("version")
                .short('v')
                .long("version")
                .action(ArgAction::SetTrue)
                .help("Get current version."),
        )
        .disable_help_flag(true) // Otherwise it is displayed in a wrong position
        .disable_version_flag(true)
        .disable_colored_help(false)
        .styles(styles)
        .after_help("NOTE: This tool is in very early development.
      If it doesn't work for you, please fill a bug report here:
      https://github.com/isbm/mezzotint/issues\n".bright_yellow().to_string())
}
