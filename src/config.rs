use std::{env, net, path::Path, process, str::FromStr};

#[derive(Debug)]
pub struct Config {
    pub fd: String,
    pub main_file: String,
    pub ip_addr: net::IpAddr,
    pub port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            fd: env::current_dir().unwrap().to_str().unwrap().to_string(),
            main_file: String::from("index.html"),
            ip_addr: net::IpAddr::V4(net::Ipv4Addr::new(127, 0, 0, 1)),
            port: 8080,
        }
    }
}

impl Config {
    pub fn update_config(&mut self, args: &Vec<String>) {
        let arg_size = args.len();
        let mut possible_file_names: Vec<String> = Vec::new();
        let mut i = 1;
        while i < arg_size {
            let arg_str = args[i].as_str();
            if args[i].as_str() == "--ip" {
                let ip_addr = match args.get(i + 1) {
                    Some(a) => a.clone(),
                    None => {
                        println!("I think you forgot to give the ip address");
                        process::exit(2);
                    }
                };

                let number_of_dot = ip_addr.matches('.').count();
                let number_of_semicolon = ip_addr.matches(':').count();
                if number_of_dot == 3 {
                    let ip_result = match net::Ipv4Addr::from_str(&ip_addr) {
                        Ok(a) => a,
                        Err(_) => {
                            println!("Unable to parse the Ip4V I am sorry");
                            process::exit(2);
                        }
                    };
                    self.ip_addr = net::IpAddr::V4(ip_result);
                } else if number_of_semicolon == 5 {
                    let ip_result = match net::Ipv6Addr::from_str(&ip_addr) {
                        Ok(a) => a,
                        Err(_) => {
                            println!("Unable to parse the Ip4V I am sorry");
                            process::exit(2);
                        }
                    };
                    self.ip_addr = net::IpAddr::V6(ip_result);
                } else {
                    println!(
                        "Sorry to say the IP address you provided is wrong\n(Try this: 127.0.0.1)"
                    );
                    process::exit(2);
                }
                i += 1;
            } else if arg_str == "--port" {
                let port_number = match args.get(i + 1) {
                    Some(a) => a,
                    None => {
                        println!("I think you forgot to give the port number");
                        process::exit(2);
                    }
                };

                let port_number = match port_number.parse::<u16>() {
                    Ok(a) => a,
                    Err(_) => {
                        println!(
                            "Unable to convert {} to a valid port number(u16)",
                            port_number
                        );
                        process::exit(2);
                    }
                };

                self.port = port_number;
                i += 1;
            } else {
                possible_file_names.push(arg_str.to_string());
            }

            i += 1;
        }

        let mut not_found = true;
        for i in possible_file_names.iter() {
            let path = Path::new(i);
            if path.exists() && path.is_file() {
                println!("{:#?}", path);
                self.main_file = path.file_name().unwrap().to_str().unwrap().to_string();
                let (relative_folder, _) = i.trim_end().rsplit_once('/').unwrap();
                if path.is_relative() {
                    self.fd = env::current_dir().unwrap().to_str().unwrap().to_string()
                        + &'/'.to_string()
                        + relative_folder;
                } else {
                    self.fd = relative_folder.to_string();
                }
                not_found = false;
                break;
            } else if path.exists() && path.is_dir() {
                let relative_folder = i.trim_end_matches('/');
                if path.is_relative() {
                    self.fd = env::current_dir().unwrap().to_str().unwrap().to_string()
                        + &'/'.to_string()
                        + relative_folder;
                } else {
                    self.fd = relative_folder.to_string();
                }
                not_found = false;
                break;
            }
        }
        if not_found {
            println!("I think someone stole your file ");
            process::exit(2);
        }
    }

    pub fn project_dir(&self) -> &Path {
        Path::new(&self.fd)
    }
}
