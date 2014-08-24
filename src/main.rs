#![experimental]

use std::comm::Messages;
use std::io::IoResult;
use std::io::TcpStream;
use std::os;
use std::sync::Future;

/// Sends as many messages as possible from the given data and returns any unsent data.
///
/// Attempts to complete any given partial message before sending the remaining messages from the
/// buffer.
fn send_messages(mut unsent: Vec<u8>, buffer: &[u8], sender: &Sender<Vec<u8>>) -> Vec<u8> {
    let mut was_cr = false;
    let mut start = 0;
    let mut end = 0;
    for (pos, byte) in buffer.iter().enumerate() {
        end = pos + 1;
        match *byte {
            b'\r' => was_cr = true,
            b'\n' => if was_cr {
                if unsent.is_empty() {
                    sender.send(Vec::from_slice(buffer.slice(start, end)));
                } else {
                    sender.send(unsent.clone().append(buffer.slice(start, end)));
                    unsent.clear();
                }

                start = end;
                was_cr = false;
            },
            _ => was_cr = false,
        }
    }

    if start != end {
        unsent.push_all(buffer.slice(start, end));
    };

    unsent
}

// TODO: Create a `Message` type and send `Result<Message, Vec<u8>>` instead of `Vec<u8>`.
/// Returns a proc that continually read from the given stream and sends CRLF delimited messages
/// from the stream on the given channel.
///
/// The proc will return an `IoResult<()>` when the stream is no longer readable for any reason.
/// If an `IoError` occured while reading from the stream it will be returned, and if zero bytes
/// are read from the stream `Ok(())` will be returned. If the stream stops being readable in
/// the middle of a message the partial message will be sent on the channel before the proc
/// returns.
///
/// Messages sent on the channel will all (with the possible exception of the last one, see above)
/// end with b"\r\n", but there is no other guarantee that they will follow the IRC specification.
///
/// If the channel's reciever is closed the proc will `fail!`.
fn read_loop(mut _stream: TcpStream, sender: Sender<Vec<u8>>) -> proc():Send -> IoResult<()> {
    proc() {
        // FIXME: The buffer size is based on max message size, can this be improved?
        let mut buffer = [0u8, ..512];
        let mut unsent = Vec::new();
        let mut ret: IoResult<()>;
        loop {
            let bytes_read = match _stream.read(buffer) {
                Ok(0) => {
                    ret = Ok(());
                    break;
                },
                Err(e) => {
                    ret = Err(e);
                    break;
                },
                Ok(i) => i,
            };

            unsent = send_messages(unsent, buffer.slice_to(bytes_read), &sender);
        }

        if !unsent.is_empty() {
            sender.send(unsent);
        }

        ret
    }
}

pub struct Connection {
    stream: TcpStream,
    receiver: Receiver<Vec<u8>>,
    future: Future<IoResult<()>>,
}

impl Connection {
    pub fn connect(host: &str, port: u16) -> IoResult<Connection> {
        let stream = try!(TcpStream::connect(host, port));
        let (tx, rx) = channel();
        let future = Future::spawn(read_loop(stream.clone(), tx));

        Ok(Connection {
            stream: stream,
            receiver: rx,
            future: future,
        })
    }

    pub fn iter<'a>(&'a self) -> Messages<'a, Vec<u8>> {
        self.receiver.iter()
    }
}

fn main() {
    let args = os::args();

    if args.len() != 2 {
        return;
    }

    let mut connection = Connection::connect(args[1].as_slice(), 6667).unwrap();

    for msg in connection.iter() {
        print!("{}", String::from_utf8_lossy(msg.as_slice()));
    }
}
