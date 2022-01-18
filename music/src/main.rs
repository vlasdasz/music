use std::{
    io::{Result, Write},
    net::TcpStream,
};

use model::device_info::DeviceInfo;

fn main() -> Result<()> {
    let info = DeviceInfo::default();

    dbg!(&info);

    let mut stream = match TcpStream::connect("127.0.0.1:57841") {
        Err(e) => panic!("Failed to connect to remote: {e}"),
        Ok(s) => s,
    };

    let ser = net::data::ser(&info);

    stream.write(&ser)?;

    println!("write OK");

    Ok(())
}
