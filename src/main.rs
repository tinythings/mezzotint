use crate::scanner::debpkg::DebPackageScanner;
use scanner::{binlib::ElfScanner, general::Scanner};
use std::{env, path::Path};

mod clidef;
mod logger;
mod scanner;

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
    let profile = params.get_one::<String>("profile");

    if exe.is_none() && profile.is_none() {
        return {
            cli.print_help().unwrap();
            Ok(())
        };
    }

    if let Some(exe) = exe {
        log::info!("Binary dependencies:");
        for d in ElfScanner::new().scan(Path::new(exe).to_owned()) {
            log::info!("  - {}", d.to_str().unwrap());
        }

        log::info!("Package dependencies:");
        for d in DebPackageScanner::new().scan(Path::new(exe).to_owned()) {
            log::info!("  - {}", d.to_str().unwrap());
        }
    }

    Ok(())
}
