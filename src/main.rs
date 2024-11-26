use core::str;
use std::{
    error,
    io::{self, BufRead, Read, Write},
    net::TcpStream,
    vec,
};

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct Query {
    class: String,
    enable: bool,
    json: bool,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default)]
struct TPV {
    class: Option<String>,
    device: Option<String>,
    status: Option<u8>,
    mode: Option<u8>,
    #[serde(alias = "time")]
    timestamp: Option<DateTime<Local>>,
    lat: Option<f64>,
    lon: Option<f64>,
    alt: Option<f64>,
    climb: Option<f64>,
    epc: Option<f64>,
    eps: Option<f64>,
    ept: Option<f64>,
    epx: Option<f64>,
    epy: Option<f64>,
    epv: Option<f64>,
    track: Option<f64>,
    speed: Option<f64>,
}

fn main() {
    // dotenvy::dotenv().ok();
    // let request_url = std::env::var("REQUEST_URL").unwrap();
    // println!("{}", request_url);
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

    let request_query = Query {
        class: "WATCH".to_string(),
        enable: true,
        json: true,
    };

    let query = format!("?WATCH={}", serde_json::to_string(&request_query)?);
    stream.write_all(query.as_bytes())?;

    let mut reader = io::BufReader::new(&stream);
    let mut buf = vec![];
    loop {
        reader.read_until(b'\n', &mut buf)?;
        let deserialized: TPV = serde_json::from_str(str::from_utf8(&buf)?)?;
        match deserialized.class {
            Some(ref class) if class == "TPV" => {
                println!("{:?}", deserialized);
            }
            _ => {}
        }
        println!("{:?}", deserialized);
        buf.clear();
    }
    Ok(())
}
