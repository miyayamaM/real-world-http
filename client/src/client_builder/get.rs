use reqwest::blocking::RequestBuilder;

pub fn get(url: &str) -> RequestBuilder {
    let client = reqwest::blocking::Client::new();

    client.get(url).query(&[("name", "hello world")])
}
