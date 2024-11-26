use std::{error, io::Read, net::TcpStream};

fn main() {
    // println!("Hello, world!");
    // // 適当なAPIにアクセスしてみる
    // let url = "https://jsonplaceholder.typicode.com/todos";
    // let body = ureq::get(&url).call().unwrap().into_string().unwrap();
    // println!("{}", body);
    // println!("Hello, world!");

    gpsd().unwrap();
}

fn gpsd() -> Result<(), Box<dyn error::Error>> {
    let gpd_addr = "127.0.0.1:2947";

    let mut stream = TcpStream::connect(gpd_addr)?;
    println!("Connected to the server!");
    let mut buff = [0; 100];
    stream.read(&mut buff)?;
    let str = buff.iter().map(|&c| c as char).collect::<String>();

    println!("{:?}", str);
    Ok(())
}
