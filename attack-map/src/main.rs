use serde::{Serialize, Deserialize};
use redis::{Client, Commands, Connection, RedisResult};
use std::env;
use chrono::{DateTime, Utc,Local, NaiveDateTime};
use reqwest;

const API:&str = "http://ip-api.com/json/";
const REDIS_URL:String = env::var("REDIS_URL").expect("REDIS_URL must be set");
const client:redis::Client = Client::open(REDIS_URL).unwrap();
const con:redis::Connection = client.get_connection().unwrap();

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Hacker {
    user: String,
    ip: String,
    lon: String,
    lat: String,
    time: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Location {
    lat: String,
    lon: String,
}


impl Default for Location {
    fn default () -> Location {
        Location{lat: "73.907".to_string(), lon: "40.7128".to_string()}
    }
}

fn pull_hackers() -> Vec<Hacker> {
    let mut res:Vec<Hacker> = vec![];

    let now:i64 = Local::now().timestamp();
    let five_hours_ago: i64 = now - 5*60*60;
    let hackers:Vec<String> = con.zrangebyscore("hackers", five_hours_ago, now).unwrap();

    for hacker in hackers {
        let hacker_json:String = con.get(hacker).unwrap();
        let hacker_struct:Hacker = serde_json::from_str(&hacker_json).unwrap();
        let time = NaiveDateTime::from_timestamp(hacker.parse::<i64>().unwrap(), 0);
        hacker_struct.time = time.format("%H:%M:%S").to_string();
        res.push(hacker_struct);
    }
    Ok(res)
}

async fn populate_redis(user: &str,ip: &str) -> Result<(), reqwest::Error>{
    let mut data: Location = Location::default();

    if con.exists(ip).unwrap() {
        println!("{} already exists", ip);
        data = serde_json::from_str(&con.get(ip).unwrap()).unwrap();

    } else {
        let url = format!("{}{}", API, ip);
        data = reqwest::get(&url).await?.json().await?;
        con.hmset(ip, &[
            ("lat", data.lat),
            ("lon", data.lon),
        ]).unwrap();
    }

    let time:i64 = Local::now().timestamp();
    con.hmset(time, &[
        ("user", user),
        ("ip", ip),
        ("lat", &data.lat),
        ("lon", &data.lon),
    ]).unwrap();

    con.zadd("hackers", time, time).unwrap();

    Ok(())
}

fn main(){

}

// fn handler(event,context) -> JsonValue {
//     method = event.requestContext.http.method;

//     if method == "GET" {
//         hackers = pull_hackers();
//         json!({
//             "statusCode": 200,
//             "body": serde_json::to_string(&hackers).unwrap()
//         })
//     } else if method == "POST" {
//         let message = serde_json::from_str(&event.body).unwrap().message;
//         parse(message);
//         json!({
//             "statusCode": 200,
//             "body": "OK"
//         })
//     } else {
//         json!({
//             "statusCode": 404,
//             "body": "Not Found"
//         })
//     }

//     let ip = reqwest::get(API+).unwrap().text().unwrap();
//     let ip = serde_json::from_str(&ip).unwrap();
// }

