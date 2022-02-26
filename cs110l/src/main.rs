use tokio::time::Duration;



#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client_builder = reqwest::Client::builder()
        .timeout(Duration::from_secs(10));
 

    let json: serde_json::Value = client_builder.build()?.get("http://ip-api.com/json/92.255.85.135").send().await?.json().await?;
        

    println!("{:#?}", json);
    // Object(
    //     {
    //         "body": String(
    //             "https://docs.rs/reqwest"
    //         ),
    //         "id": Number(
    //             101
    //         ),
    //         "title": String(
    //             "Reqwest.rs"
    //         ),
    //         "userId": Number(
    //             1
    //         )
    //     }
    // )
    Ok(())
}