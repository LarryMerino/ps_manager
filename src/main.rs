use simple_logger::SimpleLogger;
use crate::settings::Settings;

mod settings;

fn main() {
    SimpleLogger::new()
        .with_threads(true)
        .init()
        .unwrap();

    let settings = Settings::new();

    // Print out our settings
    println!("{:?}", settings);

}