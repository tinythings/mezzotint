use std::env;

mod clidef;
mod logger;

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

    Ok(())
}
