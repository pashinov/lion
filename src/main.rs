use std::str::FromStr;
use std::{thread};

use clap::{App, Arg};
use daemonize::Daemonize;
use log::{info, warn};
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use log::LevelFilter;
use signal_hook::iterator::Signals;

mod hal;
mod handler;
mod lion;
mod server;

fn run_service(settings: config::Config) -> () {
    let mut context = zmq::Context::new();

    let ctx = context.clone();
    thread::spawn(move || server::server_task(&ctx, &settings));

    let signals = Signals::new(&[signal_hook::SIGINT, signal_hook::SIGTERM]).unwrap();
    for signal in &signals {
        match signal {
            signal_hook::SIGINT | signal_hook::SIGTERM => {
                info!("Terminate program");
                context.destroy().unwrap();
                break;
            },
            _ => {
                warn!("Signal handler not found");
                continue;
            },
        }
    }
}

fn main() {
    // Parsing arguments
    let args = App::new("Lion")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Pashinov A. <pashinov93@gmail.com>")
        .arg(Arg::with_name("daemon")
            .short("d")
            .long("daemon")
            .help("Run as daemon")
            .takes_value(false))
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Sets a custom config file")
            .takes_value(true))
        .get_matches();

    // Init configuration
    let mut settings = config::Config::default();
    let config_file = args.value_of("config").unwrap_or("conf/default.json");
    settings.merge(config::File::with_name(config_file)).unwrap();

    // Init logging
    let log_filename = settings.get_str("Config.System.Logging.Path").unwrap();
    let log_level = LevelFilter::from_str(&settings.get_str("Config.System.Logging.Level").unwrap()).unwrap();

    let stdout = ConsoleAppender::builder().build();
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} [{l}] - {m}\n")))
        .build(log_filename).unwrap();

    let log_config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder()
            .appender("logfile")
            .appender("stdout")
            .build(log_level)).unwrap();

    log4rs::init_config(log_config).unwrap();

    // Run service
    if !(args.is_present("daemon")) {
        info!("Running application in console mode...");
        run_service(settings);
    } else {
        let daemon = Daemonize::new()
            .pid_file(settings.get_str("Config.System.Daemon.PidPath").unwrap())
            .working_directory("/")
            .umask(0o027)
            .privileged_action(|| info!("Running application as a daemon..."));

        match daemon.start() {
            Ok(_) => {
                info!("Running application in daemon mode...");
                run_service(settings);
            }
            Err(err) => { panic!("Running the daemon: {}", err) }
        }
    }
}
