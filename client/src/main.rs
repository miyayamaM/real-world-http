use reqwest::Error;

fn main() -> Result<(), Error> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .get("http://localhost:18888/greeting")
        .query(&[("name", "hello world")])
        .send()?;
    println!("status code: {}", response.status().as_u16());
    if let Some(message) = response.status().canonical_reason() {
        println!("status: {}", message);
    }
    for (header_key, header_value) in response.headers().iter() {
        println!("{:?}: {:?}", header_key, header_value)
    }
    println!("response: {}", response.text()?);
    Ok(())
}
