use log::{debug, error, info, log_enabled, Level};
use std::io::Read;

fn main() {
    // Initialize the logger
    env_logger::init();
    // Read dsl file
    info!("Reading dsl file");
    let mut file = std::fs::File::open("dsl.txt").unwrap();
    let mut default_code = String::new();
    file.read_to_string(&mut default_code).unwrap();
    info!("Read dsl file successfully");
    info!("Default code: {}", default_code);
}
