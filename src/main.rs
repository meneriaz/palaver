#![experimental]

use std::os;

use connection::Connection;

mod connection;

fn main() {
    let args = os::args();

    if args.len() != 3 {
        return;
    }

    let name = args[2].as_bytes();
    let mut connection = Connection::connect(args[1].as_slice(), 6667).unwrap();

    connection.send(None, b"NICK", [name]);
    connection.send(None, b"USER", [name, b"0", b"*", name]);

    for msg in connection.iter() {
        print!("{}", String::from_utf8_lossy(msg.as_slice()));
    }
}
