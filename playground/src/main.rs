use log::info;

fn main() {
    println!("Hello, world!");
    logtools::setup_logging().expect("Unable to init logger");
    info!("Hello, world!");
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
