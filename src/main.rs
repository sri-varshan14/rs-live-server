mod config;
mod server;

use std::{env, process};

fn main() {
    let mut global_config: config::Config = config::Config::default();

    let program_args: Vec<String> = env::args().collect();
    global_config.update_config(&program_args);
    match env::set_current_dir(global_config.project_dir()) {
        Ok(_) => {
            println!("CHANGED DIR TO : {}", global_config.fd);
        }
        Err(_) => {
            println!("Unable to change to the Dir : {}", global_config.fd);
            process::exit(3);
        }
    }
    println!("{}", &env::current_dir().unwrap().display());
    let mut main_server = server::Server::new(global_config);
    main_server.bind();
    main_server.handle_requests();
}
