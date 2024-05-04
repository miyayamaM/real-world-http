use std::{env, error::Error};

mod client_builder;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let client = match args[1].as_str() {
        "get" => Ok(client_builder::get()),
        "head" => Ok(client_builder::head()),
        _ => Err("Invalid method"),
    };
    let response = client?.send()?;

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
