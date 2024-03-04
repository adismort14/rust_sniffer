use std::io::{self, Write};
use std::net::{IpAddr, TcpStream};
use std::str::FromStr;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::{env, process};

// The arguments will be of three types
// ip_sniffer -h
// ip_sniffer 192.168.1.1
// ip_sniffer -m 100 192.168.1.1

struct Arguments {
    flag: String,
    ip_addr: IpAddr,
    threads: u16,
}

const MAX_PORT: u16 = 65535;

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
                return Err("-help");
            } else if user_flag.contains("-t") {
                // if let Ok(user_threads) = args[2].clone().parse::<u16>() {
                //     let f = args[3].clone();
                //     if let Ok(ipaddr) = IpAddr::from_str(&f) {
                //         return Ok(Arguments {
                //             flag: "-t".to_string(),
                //             ip_addr: ipaddr,
                //             threads: user_threads,
                //         });
                //     } else {
                //         return Err("The IP Address is invalid. Please check it again.");
                //     }
                // } else {
                //     return Err("The argument after -t flag is supposed to be the number of threads i.e. a positive integer.");
                // }

                // This is the more readable implementation of the above snippet. Try to use match as much as possible.
                let ip_addr = match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => return Err(
                        "The IP Address is invalid. Please check it again. Must be IPv4 or IPv6",
                    ),
                };
                let threads = match args[2].parse::<u16>() {
                    Ok(s) => s,
                    Err(_) => return Err("The argument after -t flag is supposed to be the number of threads i.e. a positive integer."),
                };
                return Ok(Arguments {
                    threads,
                    flag: user_flag,
                    ip_addr,
                });
            } else {
                return Err("The input format is incorrect. Check the docs with the -h flag.");
            }
        }
    }
}

fn scan(tx: Sender<u16>, start_port: u16, ip_addr: IpAddr, num_threads: u16) {
    let mut port = start_port + 1;
    loop {
        match TcpStream::connect((ip_addr, port)) {
            Ok(_) => {
                print!(".");
                // io::stdout().flush();
                tx.send(port).unwrap();
            }
            Err(_) => {}
        }

        if (MAX_PORT - port) <= num_threads {
            break;
        }
        port += num_threads;
    }
}

fn main() {
    // Variables should be in snake_case, not camel or pascal.
    let user_args: Vec<String> = env::args().collect();
    // Arguments::new(user_args);

    // Need to pass it as borrow otherwise it takes ownership implicitly.
    // for arg in &user_args {
    //     println!("{:?}", arg);
    // }

    let program = user_args[0].clone();
    let arguments = Arguments::new(&user_args).unwrap_or_else(|err: &str| {
        if err.contains("help") {
        } else {
            eprintln!("The program {} returned an error message: {}", program, err);
        }
        process::exit(0);
    });

    let num_threads = arguments.threads;
    let addr = arguments.ip_addr;
    let (tx, rx) = channel();
    for i in 0..num_threads {
        let tx = tx.clone();
        thread::spawn(move || {
            scan(tx, i, addr, num_threads);
        });
    }

    let mut out = vec![];
    drop(tx);
    for p in rx {
        out.push(p);
    }
    out.sort();
    for v in out {
        println!("{} is an open port.", v);
    }

    // To give arguments to the program and not to cargo, use cargo run -- <enter arguments here>
    // println!("{:?}", user_args);
}
