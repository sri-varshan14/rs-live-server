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

use super::config;
use std::{
    fs,
    io::prelude::*,
    net::{TcpListener, TcpStream},
    process, str,
};

pub struct Server {
    configuration: config::Config,
    listener: Option<TcpListener>,
}

impl Server {
    pub fn new(config: config::Config) -> Server {
        Server {
            configuration: config,
            listener: None,
        }
    }

    pub fn bind(&mut self) {
        self.listener = match TcpListener::bind(format!(
            "{}:{}",
            self.configuration.ip_addr, self.configuration.port
        )) {
            Ok(a) => Some(a),

            Err(_) => {
                println!("Unable to bind the TCP socket");
                process::exit(3);
            }
        };
        println!(
            "[BINDING] : ADDR=> http://{}:{}",
            self.configuration.ip_addr, self.configuration.port
        );
    }

    pub fn handle_requests(&self) {
        for stream in self.listener.as_ref().unwrap().incoming() {
            let stream = stream.unwrap();

            self.handle_connection(stream);
        }
    }

    fn handle_connection(&self, mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        let reloader_injection = format!("
            <!-- CODE INJECTED BY LIVE_SERVER -->
            <script>
                function reloader() {{
                    location.reload()
                }}
                window.setTimeout(reloader,{});
            </script>
        ",self.configuration.duration_ms);
        stream.read(&mut buffer).unwrap();

        let root_request = b"GET / HTTP/1.1\r\n";
        let filename: String = match buffer.starts_with(root_request) {
            true => self.configuration.main_file.clone(),
            false => {
                let request_str_val = str::from_utf8(&buffer).unwrap();
                let split_val: Vec<_> = request_str_val.split(' ').collect();
                if split_val.len() >= 3 {
                    format!(
                        "{}{}",
                        &self.configuration.fd,
                        split_val.get(1).unwrap().to_string()
                    )
                }
                else{
                    return;
                }
            }
        };
        let mut contest = match fs::read_to_string(&filename) {
            Ok(a) => a,
            Err(_) => {
                return;
            }
        };

        if filename == self.configuration.main_file {
            contest.push_str(&reloader_injection);
        }

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contest.len(),
            contest
        );
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
