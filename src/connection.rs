#![experimental]

use std::io::{BufferedReader, BufferedWriter};
use std::io::{IoResult, TcpStream};
use std::sync::Future;

type Connection = (Receiver<Vec<u8>>,
                   BufferedWriter<TcpStream>,
                   Future<IoResult<()>>);

// TODO: Turn this into a proper type.
type Message<'a> = (Option<&'a [u8]>,
                &'a [u8],
                &'a [&'a[u8]]);

pub fn send<T: Writer>(w: &mut T, messages: &[Message]) -> IoResult<()> {
    for msg in messages.iter() {
        let (prefix, command, params) = *msg;
        try!(buffer_message(w, prefix, command, params));
    }
    w.flush()
}

// FIXME: Temporary workaround for rust-lang/rust#16671.
#[allow(unused_mut)]
pub fn connect(host: &str, port: u16, nick: &[u8], user: &[u8], mode: &[u8],
               real: &[u8]) -> IoResult<Connection> {
    let stream = try!(TcpStream::connect(host, port));
    let (tx, rx) = channel();

    let mut reader = BufferedReader::new(stream.clone());

    // While IRC messages are actually separated by b"\r\n" this leads to a
    // lot of parsing we don't really want to do yet. It wouldn't really make
    // sense to make a lot of effort to verify that a minimal part of the
    // message format is followed.
    let future = Future::spawn(proc() {
        loop { tx.send(try!(reader.read_until(b'\n'))); }
    });

    let mut stream_writer = BufferedWriter::new(stream);

    try!(send(&mut stream_writer,
              [(None, b"NICK", &[nick]),
               (None, b"USER", &[user, mode, b"*", real])
              ]));

    Ok((rx, stream_writer, future))
}

fn buffer_message<T: Writer>(w: &mut T, prefix: Option<&[u8]>, command: &[u8],
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

    w.write(b"\r\n")
}

// XXX: The end result will be that the connection is closed.
#[allow(unused_must_use)]
pub fn close_stream(w: BufferedWriter<TcpStream>) {
    let mut stream = w.unwrap();
    stream.close_read();
    stream.close_write();
}
