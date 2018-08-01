//! A simple JSON-based echo server over TCP using tokio-jsoncodec.

extern crate serde_json;
extern crate tokio;
extern crate tokio_codec;
extern crate tokio_jsoncodec;

use tokio::net::TcpListener;
use tokio::prelude::*;
use tokio_codec::Decoder;
use tokio_jsoncodec::Codec;

fn main() -> ! {
    let listen_addr = "[::]:7777".parse().unwrap();
    let listener = TcpListener::bind(&listen_addr).unwrap();
    eprintln!("Listening on {}", listen_addr);

    tokio::run(
        listener
            .incoming()
            .for_each(|tcp_stream| {
                let (sink, stream) = Codec::default().framed(tcp_stream).split();
                tokio::spawn(
                    sink.send_all(stream.map(|data: serde_json::Value| data))
                        .map(|_| ())
                        .map_err(|e| eprintln!("error in server: {:?}", e)),
                );
                Ok(())
            })
            .map_err(|e| eprintln!("error in server: {:?}", e)),
    );
    panic!("unexpectedly exited");
}
