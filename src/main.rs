use core::str;
use std::{
    error,
    io::{self, BufRead, Read, Write},
    net::TcpStream,
    vec,
};

use chrono::{DateTime, Local, NaiveDateTime, Timelike};
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

#[derive(Debug, Serialize)]
struct RasPiRequest {
    #[serde(rename = "buspositionID")]
    busposition_id: i32,
    lat: f64,
    lon: f64,
    time: NaiveDateTime,
}

fn main() {
    // println!("Hello, world!");
    // // 適当なAPIにアクセスしてみる
    // let url = "https://jsonplaceholder.typicode.com/todos";
    // let body = ureq::get(&url).call().unwrap().into_string().unwrap();
    // println!("{}", body);
    // println!("Hello, world!");

    dotenvy::dotenv().ok();
    let request_url = std::env::var("REQUEST_URL").unwrap();
    println!("{}", request_url);

    gps(request_url).unwrap();
}

fn gps(request_url: String) -> Result<(), Box<dyn error::Error>> {
    let gpd_addr = "127.0.0.1:2947";
    let request_path = format!("{}/raspi", request_url);

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
                let req = RasPiRequest {
                    busposition_id: 1,
                    lat: deserialized.lat.unwrap(),
                    lon: deserialized.lon.unwrap(),
                    time: deserialized.timestamp.unwrap().naive_utc(),
                };
                println!("gps: {:?}", req);
                let req_with_params = format!("{}?{}", request_path, into_param_string(req));
                let res = ureq::get(&req_with_params).call();
                println!("res: {:?}", res);
                // println!("{:?}", serde_json::to_string(&req)?);
            }
            _ => {}
        }
        buf.clear();
    }
    Ok(())
}

fn into_param_string(req: RasPiRequest) -> String {
    let mut params = vec![];
    params.push(format!("buspositionID={}", req.busposition_id));
    params.push(format!("lat={}", req.lat));
    params.push(format!("lon={}", req.lon));
    let time = format!("{}", req.time.hour());
    // let time = req.time.format("%H:%M").to_string();
    params.push(format!("time={}", 9));
    params.join("&")
}
