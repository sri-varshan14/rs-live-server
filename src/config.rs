use std::{net, process, str::FromStr, path::Path, env};

#[derive(Debug)]
pub struct Config {
    fd: String,
    main_file: String,
    ip_addr: net::IpAddr,
    port: u16,
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
        for i in 1..arg_size {
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
                println!(". = {},: = {}", number_of_dot, number_of_semicolon);
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
            } else {
                possible_file_names.push(arg_str.to_string());
            }
        }

        for i in possible_file_names.iter() {
            if Path::new(&i).exists() {
                self.main_file = i.to_string();
                self.fd = 
                break;
            }
        }
    }
}
