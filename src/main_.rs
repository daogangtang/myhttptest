// fn main() {
// println!("Hello, world!");
// }


extern crate http_muncher;

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
        println!("{:?}: ", ::std::str::from_utf8(header).unwrap());

        // We have nothing to say to parser, and we'd like
        // it to continue its work - so let's return "None".
        true
    }

    // And let's print the header values in a similar vein:
    fn on_header_value(&mut self, value: &[u8]) -> bool {
        println!("\t {:?}", ::std::str::from_utf8(value).unwrap());
        true
    }
}

fn main() {
    // Now we can create a parser instance with our callbacks handler:
    let callbacks_handler = MyHandler;
    let mut parser = Parser::request(callbacks_handler);

    // Let's define a mock HTTP request:
    let http_request = "GET / HTTP/1.0\r\nContent-Type: text/plain\r\nContent-Length: 0\r\nHello: \
                        World\r\n\r\n";

    // And now we're ready to go!
    parser.parse(http_request.as_bytes());

    // Now that callbacks have been called, we can introspect
    // the parsing results - for instance, print the HTTP version:
    let (http_major, http_minor) = parser.http_version();
    println!("{}.{}", http_major, http_minor);
}

// Now execute "cargo run", and as a result you should see this output:

// Content-Type:
//   text/plain
// Content-Length:
//   0
// Hello:
//   World

// ... and the rest isf almost the same - have fun experimenting!
