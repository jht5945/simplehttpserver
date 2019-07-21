use std::{
    collections::HashMap,
    io::{Read, Write},
    net::TcpStream,
};

use rust_util::*;

#[derive(Debug)]
struct HttpRequestHeaders {
    header_list: Vec<String>,
    header_map: HashMap<String, Vec<String>>,
}

#[derive(Debug)]
struct HttpRequest {
    method: String,
    path: String,
    protocol: String,
    headers: HttpRequestHeaders,
}

#[allow(dead_code)]
impl HttpRequestHeaders {
    pub fn new() -> HttpRequestHeaders {
        HttpRequestHeaders {
            header_list: Vec::new(),
            header_map: HashMap::new(),
        }
    }

    pub fn add_header(&mut self, h: String, n: String) {
        match self.header_map.get_mut(&h) {
            Some(nl) => nl.push(n),
            None => {
                let mut nl: Vec<String> = Vec::new();
                self.header_list.push(h.clone());
                nl.push(n);
                self.header_map.insert(h, nl);
            },
        }
    }

    pub fn get_header(&self, h: &String) -> Option<&Vec<String>> {
        self.header_map.get(h)
    }

    pub fn get_header_line(&self, h: &String) -> Option<&String> {
        Some(&self.get_header(h)?[0])
    }
}

#[allow(dead_code)]
fn parse_http_request_from_read(_read: &Read) -> XResult<HttpRequest> {
    // TODO ...
    Ok(HttpRequest{
        method: "GET".to_string(),
        path: "/".to_string(),
        protocol: "HTTP/1.0".to_string(),
        headers: HttpRequestHeaders::new(),
    })
}

// TODO  parse HTTP header ...
fn handle_read(mut stream: &TcpStream) {
    let mut buf = [0u8 ;4096];
    match stream.read(&mut buf) {
        Ok(_) => {
            let req_str = String::from_utf8_lossy(&buf);
            println!("{}", req_str);
            },
        Err(e) => println!("Unable to read stream: {}", e),
    }
}

fn handle_write(mut stream: TcpStream) {
    let response = b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Hello world</body></html>\r\n";
    match stream.write(response) {
        Ok(_) => println!("Response sent"),
        Err(e) => println!("Failed sending response: {}", e),
    }
}

pub fn handle_http_request(stream: TcpStream) {
    handle_read(&stream);
    handle_write(stream);
}
