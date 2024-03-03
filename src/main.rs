use std::env;
use std::net::IpAddr;
use std::str::FromStr;

// The arguments will be of three types
// ip_sniffer -h
// ip_sniffer 192.168.1.1
// ip_sniffer -m 100 192.168.1.1

struct Arguments {
    flag: String,
    ip_addr: IpAddr,
    threads: u16,
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            // Do not println. Return an error with an error message.
            return Err("Too few arguments. Check the manual.");
        } else if args.len() > 4 {
            return Err("Too many arguments. Check the manual.");
        }
        let f = args[1].clone();
        if let Ok(ipaddr) = IpAddr::from_str(&f) {
            return Ok(Arguments {
                flag: "".to_string(),
                ip_addr: ipaddr,
                threads: 4,
            });
        } else {
            let user_flag = args[1].clone();
            if user_flag.contains("-h") || user_flag.contains("--help") {
                println!("Usage: -t to select how many threads you want it to use e.g. ip_sniffer.exe -t 16 <IP_ADDR> \r\n -h or --help to read the manual \r\n No flags and arguments will lead to default number of threads being used (4) e.g ip_sniffer <IP_ADDR>");
            } else if user_flag.contains("-t") {
                if let Ok(user_threads) = args[2].clone().parse::<u16>() {
                    let f = args[3].clone();
                    if let Ok(ipaddr) = IpAddr::from_str(&f) {
                        return Ok(Arguments {
                            flag: "-t".to_string(),
                            ip_addr: ipaddr,
                            threads: user_threads,
                        });
                    } else {
                        return Err("The IP Address is invalid. Please check it again.");
                    }
                } else {
                    return Err("The argument after -t flag is supposed to be the number of threads i.e. a positive integer.");
                }
            } else {
                return Err("The input format is incorrect. Check the docs with the -h flag.");
            }
        }

        return Err("This is the right range but fuck it, here is another error.");
    }
}

fn main() {
    // Variables should be in snake_case, not camel or pascal.
    let user_args: Vec<String> = env::args().collect();
    // Arguments::new(user_args);

    // Need to pass it as borrow otherwise it takes ownership implicitly.
    for arg in &user_args {
        println!("{:?}", arg);
    }

    // To give arguments to the program and not to cargo, use cargo run -- <enter arguments here>
    println!("{:?}", user_args);
}
