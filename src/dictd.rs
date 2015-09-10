use std::error::Error;
use std::io::{BufRead, Write};
use std::thread;
use std::net::{TcpListener, TcpStream};
use std::io::BufReader;

use ::page;

/// modified from gkbrk's demo:
/// https://gist.github.com/gkbrk/bea6dee7c0478395b718

fn handle_client(mut stream: TcpStream) {
let mut reader = BufReader::new(stream.try_clone().unwrap());
    stream.write("220 urdict 0.1.0\r\n".as_bytes()).unwrap();
    loop {
        let mut line = String::new();
        let line_len = reader.read_line(&mut line).unwrap();
        if line_len > 0{
            println!("{}", line);
            let pieces: Vec<&str> = line.trim().split(" ").collect();
            match pieces[0].to_lowercase().as_ref() {
                "define" => {
                    let word = pieces[2].replace("\"", "");
                    println!("{}", word);

                    if let Some(def) = page::find_on_urban_dict(&word) {
                        stream.write(format!("150 {} definitions retrieved\r\n", 1).as_bytes()).unwrap();

                        stream.write(format!("151 \"{}\" gcide", def.word).as_bytes()).unwrap();
                        stream.write(format!(" \"Urban Dictionary {} {}\"\r\n", def.contributor, def.date).as_bytes()).unwrap();
                        stream.write(format!("{}\r\n.\r\n", def.def).as_bytes()).unwrap();

                        stream.write("250 ok\r\n".as_bytes()).unwrap();
                        println!("found {}", word);
                    } else {
                        //TODO
                    }
                },
                "quit" => {
                    stream.write("221 bye\r\n".as_bytes()).unwrap();
                    break;
                },
                "client" => {
                    stream.write("250 ok\r\n".as_bytes()).unwrap();
                }
                _ => {
                    println!("{}", line.trim());
                }
            }
        }else{
            println!("Client disconnected");
            break;
        }
    }
}

pub fn start_server(host: &str, port: u16) {
    let listener = TcpListener::bind(&(host, port)).unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                    handle_client(stream);
                });
            },
            Err(e) => {
                println!("{}", e.description());
            }
        }
    }
}
