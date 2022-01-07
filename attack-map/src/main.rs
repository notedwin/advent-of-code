use serde::{Serialize, Deserialize};
use redis::{Client, Commands, Connection, RedisResult};
use std::env;
use std::collections::HashMap;
use std::time::{SystemTime, Duration};
use chrono::offset::Utc;
use chrono::DateTime;


const API:&str = "http://ip-api.com/json/";
const REDIS_URL:String = env::var("REDIS_URL").expect("REDIS_URL must be set");
const client:redis::Client = Client::open(REDIS_URL).unwrap();
const con:redis::Connection = client.get_connection().unwrap();

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    message: String,
}

fn pull_hackers() -> RedisResult<()> {
    let now = SystemTime::now();

    let five_hours = Duration::from_secs(5*60*60);
    let five_hours_ago = now - five_hours;
    let hackers = con.zrange("hackers", five_hours_ago,now.to_secs() ).unwrap();
    let hackers: Vec<String> = hackers.iter().map(|h| h.unwrap()).collect();
    for hack in hackers {
        let hack = serde_json::from_str(&hack).unwrap();
        println!("{}", hack.message);
    }
    Ok(())
}

fn populate_redis(user: &str,ip: &str, r: &mut redis::Connection) -> RedisResult<()> {
    if r.exists(ip).unwrap() {
        println!("{} already exists", ip);
    } else {
        //r.set(user: &str, ip: &str).unwrap();
        println!("{} added", ip);
    }


    Ok(())
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

