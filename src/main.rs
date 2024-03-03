use std::env;
use std::net::IpAddr;

struct Arguments{
    flag: String,
    ip_addr: IpAddr,
    threads: u16,
}

fn main() {
    // Variables should be in snake_case, not camel or pascal.
    let user_args:Vec<String>=env::args().collect();

    // Need to pass it as borrow otherwise it takes ownership implicitly.
    for arg in &user_args{
        println!("{:?}",arg);
    }

    

    // To give arguments to the program and not to cargo, use cargo run -- <enter arguments here>
    println!("{:?}",user_args);
}
