mod config;

use std::env;

fn main() {
    let mut global_config: config::Config = config::Config::default();

    let program_args: Vec<String> = env::args().collect();
    global_config.update_config(&program_args);
    println!("{:#?}", global_config);
}

