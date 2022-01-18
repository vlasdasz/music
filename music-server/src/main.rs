use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

use id3::{Tag, TagLike};
use model::device_info::DeviceInfo;
use tools::{file::File, paths::Paths};

fn handle_client(mut stream: TcpStream) {
    dbg!(&stream);

    let mut buf = [0; 128];

    stream.read(&mut buf).unwrap();

    let device_info: DeviceInfo = net::data::de(&buf);

    dbg!(device_info);
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Hello");

    check_local();

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

fn check_local() {
    let home = Paths::home();
    let music_path = home.join(".music-server");

    dbg!(&music_path);

    File::mkdir(&music_path);

    let songs = File::get_files(music_path);

    dbg!(&songs);

    for song in songs {
        let tag = Tag::read_from_path(song);
        dbg!(&tag);
    }
}
