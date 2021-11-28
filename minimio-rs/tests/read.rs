extern crate minimio_rs;
use std::{
    collections::HashMap,
    io::{self, Read, Write},
    net::TcpStream,
};

use minimio_rs::{Event, Interest, Poll, Token};

#[test]
fn read() -> io::Result<()> {
    let mut poll = Poll::new()?;
    println!("queue_fd: {}", poll.registry().selector.kqfd);

    let mut streams: HashMap<Token, TcpStream> = HashMap::new();

    let address = "flash.siwalik.in";
    let mut event_count = 5;
    for i in (0..event_count).rev() {
        let mut stream = TcpStream::connect(format!("{}:8080", address)).unwrap();

        let request = format!(
            "GET /delay/{} HTTP/1.1\r\n\
                Host: {}\r\n\
                Connection: close\r\n\
                \r\n",
            i * 1000,
            address
        );

        println!("request: {}", request);
        stream.write_all(request.as_bytes())?;
        poll.registry().register(&stream, i, Interest::READABLE)?;
        streams.insert(i, stream);
    }

    println!("register done!");

    let mut events: Vec<Event> = Vec::with_capacity(event_count);
    while event_count > 0 {
        poll.poll(&mut events, None)?;
        println!("events: {:#?}", events);

        for event in events.iter() {
            let token = event.token();
            let mut stream: TcpStream = streams.remove(&token).unwrap();
            let mut res = String::new();
            stream.read_to_string(&mut res)?;
            println!("{}", res);
        }
        event_count -= events.len();
    }

    return Ok(());
}
