#[macro_use]
extern crate clap;

use base64;
use clap::App;
use regex::bytes::Regex;
use sha1::{Digest, Sha1};
use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::process::{Command, Stdio};

extern crate server;
use server::config::Config;
use server::thread_pool::ThreadPool;

fn main() {
    let app = App::new("clip-studio-remote")
        .version(crate_version!())
        .author(crate_authors!());

    let config = Config::parse(app);
    #[cfg(debug_assertions)]
    println!("config: {:?}", config);

    let listener = TcpListener::bind(format!("127.0.0.1:{}", config.port)).unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(move || {
            handle_connection(stream, &config);
        });
    }
}

fn handle_connection(mut stream: TcpStream, config: &Config) {
    let mut buf = [0; 1024];
    stream.read(&mut buf).unwrap();
    // println!("{}", String::from_utf8_lossy(&buf));

    let get_index = b"GET / HTTP/1.1\r\n";
    let get_js = b"GET /bundle.js HTTP/1.1\r\n";
    let get_css = b"GET /style.css HTTP/1.1\r\n";
    let websocket = b"GET /websocket HTTP/1.1\r\n";

    if buf.starts_with(get_index) {
        serve_index_page(&mut stream);
    } else if buf.starts_with(get_css) {
        serve_css(&mut stream);
    } else if buf.starts_with(get_js) {
        serve_js(&mut stream);
    } else if buf.starts_with(websocket) {
        if let Some(key) = parse_ws_key(&buf) {
            send_back_handshake(&mut stream, key);
            receive_ws_messages(&mut stream, config);
        } else {
            serve_404_page(&mut stream);
        }
    } else {
        serve_404_page(&mut stream);
    }
}

fn serve_index_page(stream: &mut TcpStream) {
    let response = format!(
        "HTTP/1.1 200 OK\r\n\
         Content-Type: text/html\r\n\r\n{}",
        fs::read_to_string("../client/index.html").unwrap()
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn serve_css(stream: &mut TcpStream) {
    let response = format!(
        "HTTP/1.1 200\r\n\
         Content-Type: text/css\r\n\r\n{}",
        fs::read_to_string("../client/style.css").unwrap()
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn serve_js(stream: &mut TcpStream) {
    let response = format!(
        "HTTP/1.1 200\r\n\
         Content-Type: text/javascript\r\n\r\n{}",
        fs::read_to_string("../client/bundle.js").unwrap()
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn serve_404_page(stream: &mut TcpStream) {
    let response = format!(
        "HTTP/1.1 404 NOT FOUND\r\n\
         Content-Type: text/html\r\n\r\n{}",
        fs::read_to_string("../client/404.html").unwrap()
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn parse_ws_key<'a>(headers: &'a [u8]) -> Option<String> {
    let regex = Regex::new("Sec-WebSocket-Key: (.*)").unwrap();
    regex
        .captures(headers)
        .and_then(|caps| caps.get(1))
        .map(|m| String::from(String::from_utf8_lossy(m.as_bytes()).trim()))
}

fn send_back_handshake(stream: &mut TcpStream, key: String) {
    let key = key + "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";
    let hash = base64::encode(Sha1::digest(key.as_bytes()));

    let response = format!(
        "HTTP/1.1 101 Switching Protocols\r\n\
         Connection: Upgrade\r\n\
         Upgrade: websocket\r\n\
         Sec-WebSocket-Accept: {}\r\n\r\n",
        hash
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn receive_ws_messages(stream: &mut TcpStream, config: &Config) {
    let mut child_stdin = if cfg!(target_os = "macos") {
        let mut command = Command::new("osascript");
        let command =
            command
                .stdin(Stdio::piped())
                .stdout(Stdio::null())
                .args(&["-l", "JavaScript", "-i"]);
        let child_stdin = if let Ok(child_process) = command.spawn() {
            child_process.stdin.unwrap()
        } else {
            panic!("Faild to start a process for executing osascript")
        };
        Some(child_stdin)
    } else {
        None
    };

    loop {
        let mut msg_buf = [0; 1024];
        if stream.read(&mut msg_buf).is_ok() {
            if msg_buf[0] == 0 {
                break;
            }
            let opcode = msg_buf[0] % 16;
            if opcode == 1 {
                let payload_length = (msg_buf[1] % 128) as usize;
                let mask: Vec<u8> = msg_buf[2..=5].to_vec();

                let mut payload = Vec::<u8>::with_capacity(payload_length);
                for i in 0..payload_length {
                    payload.push(msg_buf[6 + i] ^ mask[i % 4]);
                }
                let payload = String::from_utf8(payload).unwrap();
                let payload = payload.trim();

                if cfg!(target_os = "macos") && config.send_keys {
                    child_stdin
                        .as_mut()
                        .unwrap()
                        .write(
                            format!("Application('System Events').keystroke('{}')\n", payload)
                                .as_ref(),
                        )
                        .unwrap();
                } else {
                    #[cfg(debug_assertions)]
                    println!("Received: {}", payload);
                }
            } else if opcode == 9 {
                #[cfg(debug_assertions)]
                println!("Pong");
                stream.write(&[138, 0]).unwrap();
                stream.flush().unwrap();
            } else {
                eprintln!("Unsupported opcode {}; ignoring.", opcode);
            }
        } else {
            break;
        }
    }
}
