use reqwest::blocking::RequestBuilder;

pub fn head(url: &str) -> RequestBuilder {
    let client = reqwest::blocking::Client::new();

    client.head(url).query(&[("name", "hello world")])
}
