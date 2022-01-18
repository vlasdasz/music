use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

fn handle_client(mut stream: TcpStream) {
    dbg!(&stream);

    let mut buf = [0; 128];

    stream.read(&mut buf).unwrap();

    dbg!(buf);
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Hello");

    let listener = TcpListener::bind("127.0.0.1:57841")?;

    println!("Listening...");

    // accept connections and process them serially
    for stream in listener.incoming() {
        tokio::spawn(async move {
            handle_client(stream.unwrap());
        });
    }
    Ok(())
}