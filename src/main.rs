mod clidef;
mod filters;
mod logger;
mod procdata;
mod profile;
mod scanner;

use crate::{
    filters::{dirs::PathsDataFilter, intf::DataFilter, texts::TextDataFilter},
    profile::Profile,
    scanner::debpkg::DebPackageScanner,
};
use scanner::{binlib::ElfScanner, general::Scanner};
use std::{
    collections::HashSet,
    env,
    path::{Path, PathBuf},
    process,
};

static VERSION: &str = "0.1";
static LOGGER: logger::STDOUTLogger = logger::STDOUTLogger;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let mut cli = clidef::cli(VERSION);

    if args.len() == 1 {
        return {
            cli.print_help().unwrap();
            Ok(())
        };
    }

    let params = cli.to_owned().get_matches();

    // Setup logger
    if let Err(err) = log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(if params.get_flag("debug") { log::LevelFilter::Trace } else { log::LevelFilter::Info }))
    {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, err.to_string()));
    }

    let exe = params.get_one::<String>("exe");
    let profile_path = params.get_one::<String>("profile");

    if exe.is_none() && profile_path.is_none() {
        return {
            cli.print_help().unwrap();
            Ok(())
        };
    }

    if let Some(exe) = exe {
        let mut paths: HashSet<PathBuf> = HashSet::default();

        log::info!("Find binary dependencies");
        paths.extend(ElfScanner::new().scan(Path::new(exe).to_owned()));

        log::info!("Find package dependencies");
        paths.extend(DebPackageScanner::new().scan(Path::new(exe).to_owned()));

        log::info!("Filtering satellite data");

        log::info!("Filtered path data:");
        for p in TextDataFilter::new(PathsDataFilter::new(paths.into_iter().collect::<Vec<PathBuf>>()).filter())
            .remove_manpages()
            .remove_docs()
            .remove_l10n()
            .filter()
        {
            log::info!("  - {}", p.to_str().unwrap());
        }
    } else if let Some(profile_path) = profile_path {
        log::info!("Getting profile at {profile_path}");
        match Profile::new(Path::new(profile_path)) {
            Ok(profile) => {
                if let Err(err) = procdata::TintProcessor::new().set_profile(profile).start() {
                    log::error!("{}", err);
                    process::exit(exitcode::IOERR);
                }
            }
            Err(err) => {
                log::error!("{}", err);
                process::exit(exitcode::OSERR);
            }
        }
    }

    Ok(())
}
