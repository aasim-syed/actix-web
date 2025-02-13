use std::error::Error as StdError;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn StdError>> {
    std::env::set_var("RUST_LOG", "actix_http=trace");
    env_logger::init();

    let client = awc::Client::new();

    // Create request builder, configure request and send
    let mut response = client
        .get("https://www.rust-lang.org/")
        .append_header(("User-Agent", "Actix-web"))
        .send()
        .await?;

    // server http response
    println!("Response: {:?}", response);

    // read response body
    let body = response.body().await?;
    println!("Downloaded: {:?} bytes", body.len());

    Ok(())
}
