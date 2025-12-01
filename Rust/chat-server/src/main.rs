use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, BufReader, Write};
fn main()-> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Chat server listening on 127:0.0.1:8080");

    let (stream, addr) = listener.accept()?;
    println!("New connection from {}", addr);

    Ok(())
}
