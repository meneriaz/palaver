#![experimental]

use std::os;

use connection::{connect, close_stream};

mod connection;

fn main() {
    let args = os::args();

    if args.len() != 3 {
        return;
    }

    let name = args[2].as_bytes();

    let (rx, sw, mut ft) = connect(args[1].as_slice(), 6667,
                                   name, name, b"0", name).unwrap();

    loop {
        select! {
            msg = rx.recv_opt() => {
                match msg {
                    Ok(m) => print!("{}", String::from_utf8_lossy(m.as_slice())),
                    Err(e) => { println!("Error: {}", e); break; }
                }
            }
        }
    }

    close_stream(sw);

    match ft.get() {
        Err(e) => println!("Error: {}", e),
        _ => (),
    }

    println!("END");
}
