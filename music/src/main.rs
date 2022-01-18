use std::{
    env,
    io::{Read, Result, Write},
    net::TcpStream,
};

use model::device_info::DeviceInfo;

fn main() -> Result<()> {

    let info = DeviceInfo::default();

    dbg!(&info);

    let remote = match env::var("MUSIC_REMOTE") {
        Err(_) => panic!("Failed to get remote. Please set MUSIC_REMOTE environment variable"),
        Ok(r) => r,
    };

    println!("Remote: {remote}");

    let mut stream = match TcpStream::connect("127.0.0.1:57841") {
        Err(e) => panic!("Failed to connect to remote: {e}"),
        Ok(s) => s,
    };

    println!("Connected");

    let ser = bincode::serialize(&info).unwrap();

    dbg!(ser.len());

    dbg!(&ser);

    stream.write(&ser)?;

    println!("write OK");

    stream.read(&mut [0; 128])?;

    println!("read OK");

    Ok(())
}
