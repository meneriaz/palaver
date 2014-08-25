#![experimental]

use std::io::{BufferedReader, BufferedWriter};
use std::io::{IoResult, TcpStream};
use std::sync::Future;

pub struct Connection(
    pub Receiver<Vec<u8>>,
    pub BufferedWriter<TcpStream>,
    pub Future<IoResult<()>>
);

impl Connection {
    // FIXME: Temporary workaround for rust-lang/rust#16671.
    #[allow(unused_mut)]
    pub fn connect(host: &str, port: u16) -> IoResult<Connection> {
        let stream = try!(TcpStream::connect(host, port));
        let (tx, rx) = channel();

        let mut reader = BufferedReader::new(stream.clone());
        let future = Future::spawn(proc() {
            loop { tx.send(try!(reader.read_until(b'\n'))); }
        });

        let stream_writer = BufferedWriter::new(stream);

        Ok(Connection(rx, stream_writer, future))
    }
}

pub fn send<T: Writer>(w: &mut T, prefix: Option<&[u8]>, command: &[u8],
                       params: &[&[u8]]) -> IoResult<()> {
    match prefix {
        Some(p) => {
            try!(w.write(p));
            try!(w.write(b" "));
        }
        None => (),
    }

    try!(w.write(command));

    for param in params.init().iter() {
        try!(w.write(b" "));
        try!(w.write(*param));
    }

    match params.last() {
        Some(param) => {
            try!(w.write(b" :"));
            try!(w.write(*param));
        }
        None => (),
    }

    try!(w.write(b"\r\n"));

    w.flush()
}

// XXX: The end result will be that the connection is closed.
#[allow(unused_must_use)]
pub fn close_stream(w: BufferedWriter<TcpStream>) {
    let mut stream = w.unwrap();
    stream.close_read();
    stream.close_write();
}
