use reqwest::Error;

fn main() -> Result<(), Error> {
    let response = reqwest::blocking::get("http://localhost:18888/greeting")?;
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
