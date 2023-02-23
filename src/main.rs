/*
 *  Author : sri-varshan14
 *  Date : 21-02-2023
 *  Main Repository : github.com/sri-varshan14/rs-live-server.git
 *
 *  MIT License
 *
 *  Copyright (c) 2023 Srivarshan
 *
 *  Permission is hereby granted, free of charge, to any person obtaining a copy
 *  of this software and associated documentation files (the "Software"), to deal
 *  in the Software without restriction, including without limitation the rights
 *  to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 *  copies of the Software, and to permit persons to whom the Software is
 *  furnished to do so, subject to the following conditions:
 *
 *  The above copyright notice and this permission notice shall be included in all
 *  copies or substantial portions of the Software.
 *
 *  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 *  IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 *  FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 *  AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 *  LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 *  OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 *  SOFTWARE.
 *
 *
 * */

mod config;
mod server;
mod result;

use std::{env, process};


fn main() {
    // Global varible to store the configuration of the program
    let mut global_config: config::Config = config::Config::default();

    // To handle the arg that are passed to the program and update the global config
    let program_args: Vec<String> = env::args().collect();
    global_config.update_config(&program_args);


    match env::set_current_dir(global_config.project_dir()) {
        Ok(_) => {
            println!("[CD]: {} - [PASSED]", global_config.fd);
        }
        Err(_) => {
            println!("{}", result::err_msg(result::ErrorKind::ErrCWD));
            process::exit(3);
        }
    }

    // Creating the server and starting the live server
    let mut main_server = server::Server::new(global_config);
    main_server.bind();
    main_server.handle_requests();
}
