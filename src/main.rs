use reqwest::Error;

async fn send_request() -> Result<(), Error> {
    let response = reqwest::get("https://www.baidu.com").await?;

    // Assuming the response is in JSON format
    let body = response.text().await?;

    println!("Response body: {}", body);

    Ok(())
}

#[tokio::main]
async fn send() {
    if let Err(err) = send_request().await {
        eprintln!("Error: {}", err);
    }
}

fn main() {
    println!("Hello, world!");
    send()
}
