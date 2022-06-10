use log::info;
use std::env;
use tcpserver::run as run_tcp_server;

fn main() {
    logtools::setup_logging().expect("Unable to init logger");
    info!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    if let Ok(config) = config::Config::new(&args) {
        info!("Got config: {config:?}");
    } else {
        panic!("Unable to get config");
    }

    run_tcp_server();
}

mod config {

    #[derive(Debug)]
    #[allow(dead_code)]
    pub struct Config {
        arg0: String,
        arg1: String,
    }

    impl Config {
        pub fn new(args: &[String]) -> Result<Config, &'static str> {
            match args.len() {
                1 => Ok(Config {
                    arg0: args[0].clone(),
                    arg1: String::from(""),
                }),
                2 => Ok(Config {
                    arg0: args[0].clone(),
                    arg1: args[1].clone(),
                }),
                _ => Err("Invalid number of arguments"),
            }
        }
    }
}

mod logtools {
    pub fn setup_logging() -> Result<(), fern::InitError> {
        fern::Dispatch::new()
            // Perform allocation-free log formatting
            .format(|out, message, record| {
                out.finish(format_args!(
                    "{}[{}][{}] {}",
                    chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                    record.target(),
                    record.level(),
                    message
                ))
            })
            .level(log::LevelFilter::Debug) // Add blanket level filter -
            .level_for("hyper", log::LevelFilter::Info) // - and per-module overrides
            .chain(std::io::stdout())
            //.chain(fern::log_file("output.log")?)
            .apply()?; // Apply globally

        Ok(())
    }
}
