extern crate argparse;
extern crate term;

mod util;
mod http;

use std::{
    net::TcpListener,
    thread,
};

use argparse::{ArgumentParser, StoreTrue, Store};
use util::*;
use http::*;

const VERSION: &str = "0.1";
const DEFAULT_ADDRESS: &str = "127.0.0.1";
const DEFAULT_PORT: u16 = 8080u16;

fn print_version() {
    print!(r#"simplehttpserver {}

Copyright (C) 2019 Hatter Jiang.
License MIT <https://opensource.org/licenses/MIT>
Written by Hatter Jiang
"#, VERSION);
}


fn main() {
    let mut version = false;
    let mut bind_address = String::from(DEFAULT_ADDRESS);
    let mut bind_port = DEFAULT_PORT;
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("simplehttpserver - command line simplehttpserver tool.");
        ap.refer(&mut bind_address).add_option(&["-b", "--address"], Store, "Bind address, default 127.0.0.1");
        ap.refer(&mut bind_port).add_option(&["-p", "--port"], Store, "Bind port, default 8080");
        ap.refer(&mut version).add_option(&["-v", "--version"], StoreTrue, "Print version");
        ap.parse_args_or_exit();
    }
    
    if version {
        print_version();
        return;
    }

    let listener = match TcpListener::bind(&format!("{}:{}", &bind_address, &bind_port)) {
        Err(err) => {
            print_message(MessageType::ERROR, &format!("Listening for connections on {}:{} failed: {}",&bind_address, &bind_port, err));
            return;
        },
        Ok(listener) => {
            print_message(MessageType::OK, &format!("Listening for connections on {}:{}",&bind_address, &bind_port));
            listener
        },
    };

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_http_request(stream)
                });
            },
            Err(err) => {
                print_message(MessageType::ERROR, &format!("Unable to connect: {}", err));
            },
        }
    }
}
