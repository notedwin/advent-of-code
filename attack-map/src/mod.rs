use chrono::{Local, NaiveDateTime};
use redis::{Client, Commands, RedisResult};
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::env;



impl Default for Location {
    fn default() -> Location {
        Location {
            lat: "73.907".to_string(),
            lon: "40.7128".to_string(),
        }
    }
}

fn pull_hackers(con:&mut redis::Connection) -> Vec<Hacker> {
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
            let hacker_json: String = con.get(hacker).unwrap();
            let mut hacker_struct: Hacker = serde_json::from_str(&hacker_json).unwrap();
            let time = NaiveDateTime::from_timestamp(hacker.parse::<i64>().unwrap(), 0);
            hacker_struct.time = time.format("%H:%M:%S").to_string();
            hacker_struct
        })
        .collect();
    result
}

async fn populate_redis(user: &str, ip: &str, con: &mut redis::Connection) -> Result<(), reqwest::Error> {
    let mut data: Location = Location::default();

    if con.exists(ip).unwrap() {
        println!("{} already exists", ip);
        let loc: String = con.get(ip).unwrap();
        data = serde_json::from_str(&loc).unwrap();
    } else {
        let url = format!("{}{}", API, ip);
        data = reqwest::get(&url).await?.json().await?;
        let res: RedisResult<String> = con.hset_multiple(
            ip, 
            &[
                ("lat", &data.lat), 
                ("lon", &data.lon),
            ],
        );
        if let Err(error) = res {
            println!("{:?}", error);
        }
    }

    let time: i64 = Local::now().timestamp();
    let res: RedisResult<String> = con.hset_multiple(
        time,
        &[
            ("user", user),
            ("ip", ip),
            ("lat", &data.lat),
            ("lon", &data.lon),
        ],
    );

    if let Err(error) = res {
        println!("{:?}", error);
    }

    let res2: RedisResult<String> = con.zadd("hackers", time, time);

    if let Err(error) = res2 {
        println!("{:?}", error);
    }


    Ok(())
}

fn parseData(message: Message, con: &mut redis::Connection) {
    let mut data_arr = message.message.split(" ");
    let user: &str;
    let ip: &str;

    if data_arr.nth(3).unwrap() == "root" && data_arr.nth(3).unwrap() == "ubuntu" {
        user = data_arr.nth(3).unwrap();
        ip = data_arr.nth(5).unwrap();
    } else {
        user = data_arr.nth(5).unwrap();
        ip = data_arr.nth(7).unwrap();
    }
    populate_redis(user, ip, con);
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = handler_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

