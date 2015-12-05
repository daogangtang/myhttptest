#![feature(rustc_private)]

extern crate clap;
#[macro_use]
extern crate log;
extern crate env_logger;

extern crate coio;
extern crate hyper;
extern crate http_muncher;

use hyper::server::Response;
use hyper::header::Headers;


use clap::{Arg, App};

use coio::Scheduler;
use coio::net::tcp::TcpListener;
use coio::net::tcp::Shutdown;

// Include the 2 main public interfaces of the crate
use http_muncher::{Parser, ParserHandler};

// Now let's define a new listener for parser events:
struct MyHandler;
impl ParserHandler for MyHandler {

    // Now we can define our callbacks here.
    //
    // Let's try to handle headers: the following callback function will be
    // called when parser founds a header in the HTTP stream.

    fn on_header_field(&mut self, header: &[u8]) -> bool {
        // Print the received header key
        // println!("{:?}: ", ::std::str::from_utf8(header).unwrap());

        true
    }

    // And let's print the header values in a similar vein:
    fn on_header_value(&mut self, value: &[u8]) -> bool {
        // println!("\t {:?}", ::std::str::from_utf8(value).unwrap());
        true
    }
}

fn main() {
    env_logger::init().unwrap();

    let matches = App::new("coio-tcp-echo")
                      .version(env!("CARGO_PKG_VERSION"))
                      .author("Y. T. Chung <zonyitoo@gmail.com>")
                      .arg(Arg::with_name("BIND")
                               .short("b")
                               .long("bind")
                               .takes_value(true)
                               .required(true)
                               .help("Listening on this address"))
                      .arg(Arg::with_name("THREADS")
                               .short("t")
                               .long("threads")
                               .takes_value(true)
                               .help("Number of threads"))
                      .get_matches();

    let bind_addr = matches.value_of("BIND").unwrap().to_owned();



    let mut resp = String::new();
    resp.push_str("HTTP/1.1 200 OK\r\n");
    //resp.push_str("Connection: close\r\n");
    resp.push_str("Content-Length: 10\r\n");
    resp.push_str("\r\n");
    resp.push_str("abcdefghij");
    

    Scheduler::new()
        .with_workers(matches.value_of("THREADS").unwrap_or("1").parse().unwrap())
        .run(move || {
            let server = TcpListener::bind(&bind_addr[..]).unwrap();

            info!("Listening on {:?}", server.local_addr().unwrap());

            for stream in server.incoming() {
                use std::io::{Read, Write};


                let (mut stream, addr) = stream.unwrap();
                info!("Accept connection: {:?}", addr);

                let resptext = resp.clone();
                Scheduler::spawn(move || {
                    let mut buf = [0; 1024 * 16];

                    // Now we can create a parser instance with our callbacks handler:
                    let callbacks_handler = MyHandler;
                    let mut parser = Parser::request(callbacks_handler);

                    loop {
                        debug!("Trying to Read...");
                        match stream.read(&mut buf) {
                            Ok(0) => {
                                debug!("EOF received, going to close");
                                //stream.shutdown(Shutdown::Both);
                                break;
                            }
                            Ok(len) => {
                                info!("Read {} bytes, echo back!", len);

                                parser.parse(&buf[0..len]);
                                
                                if let Err(err) = stream.write_all(resptext.as_bytes()) {
                                    println!("write error.");
                                    break;
                                }
                                
                            }
                            Err(err) => {
                                //stream.shutdown(Shutdown::Both);
                                // panic!("Error occurs: {:?}", err);
                                break;
                            }
                        }
                    }

                    info!("{:?} closed", addr);
                });
            }
        })
        .unwrap();
}



// fn main() {
// println!("Hello, world!");
// }
