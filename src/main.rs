use byteorder;
use byteorder::LittleEndian;
use byteorder::ReadBytesExt;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

#[derive(PartialEq, PartialOrd, Debug)]
struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        println!("Connection established!");

        loop {
            let channel = stream.read_u8().unwrap();
            let command = stream.read_u8().unwrap();
            let size: u16 = stream.read_u16::<LittleEndian>().unwrap();

            let mut data = vec![0u8; size as usize];
            stream.read_exact(&mut data).unwrap();

            let m: Vec<Color> = data
                .chunks_exact(3)
                .map(|v| Color {
                    r: v[0],
                    g: v[1],
                    b: v[2],
                })
                .collect();

            println!("{:?}", m);
        }
    }
}
