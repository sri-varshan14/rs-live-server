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
        let reloader_injection = "
            <script>
                function reloader() {
                    location.reload()
                }
                window.setTimeout(reloader,3000);
            </script>
        ";
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
                println!("Unable to Find the file :{}", filename);
                return;
            }
        };

        if filename == self.configuration.main_file {
            contest.push_str(reloader_injection);
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
