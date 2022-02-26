use chrono::{Local, NaiveDateTime};
use lambda_runtime::Error;
use redis::{Client, Commands, RedisResult, Value};
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::json;

const API: &str = "http://ip-api.com/json/";
#[derive(Debug, Deserialize, Default)]
struct Args {
    #[serde(default)]
    message: String,
}

struct Hset {
    json: Hacker,
}

#[derive(Serialize, Deserialize, Debug)]
struct Hacker {
    user: String,
    ip: String,
    lon: f64,
    lat: f64,
    time: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Location {
    lat: f64,
    lon: f64,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let redis_url = "redis://127.0.0.1:6379";
    let mut con: redis::Connection = Client::open(redis_url)?.get_connection()?;
    let user = "pineapple";
    let ip = "63.46.102.5";

    let url = format!("{}{}", API, ip);
    println!("url: {}", url);
    let data: Location = reqwest::Client::new()
        .get(&url)
        .send()
        .await?
        .json()
        .await?;

    let time: i64 = Local::now().timestamp();

    let hacker = Hacker {
        user: user.to_string(),
        ip: ip.to_string(),
        lon: data.lon,
        lat: data.lat,
        time: time.to_string(),
    };

    let hacker_json: String = serde_json::to_string(&hacker).unwrap();
    println!("{}, json, {}", time.to_string(), hacker_json);

    let res: RedisResult<Value> = con.set(time.to_string(), hacker_json);

    if let Err(error) = res {
        println!("Storing time event {:?}", error);
    }

    let res2: RedisResult<Value> = con.zadd("hackers", time, time);

    if let Err(error) = res2 {
        println!("adding event to time array {:?}", error);
    }

    let hackers: Vec<Hacker> = pull_hackers(&mut con);
    let hackers_json: String = serde_json::to_string(&hackers).unwrap();
    println!("hackers: {}", hackers_json);
    Ok(())
}

fn pull_hackers(con: &mut redis::Connection) -> Vec<Hacker> {
    let now: i64 = Local::now().timestamp();
    let five_hours_ago: i64 = now - 5 * 60 * 60;

    let result: Vec<Hacker> = redis::cmd("zrangebyscore")
        .arg("hackers")
        .arg(five_hours_ago)
        .arg(now)
        .query::<Vec<String>>(con)
        .unwrap()
        .iter()
        .map(|hacker| {
            print!("hacker: {}", hacker);
            let hacker_json: String = con.get(hacker).unwrap();
            let mut hacker_struct: Hacker = serde_json::from_str(&hacker_json).unwrap();
            let time = NaiveDateTime::from_timestamp(hacker.parse::<i64>().unwrap(), 0);
            hacker_struct.time = time.format("%H:%M:%S").to_string();
            hacker_struct
        })
        .collect();
    result
}
