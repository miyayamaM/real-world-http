use reqwest::blocking::RequestBuilder;

pub fn get() -> RequestBuilder {
    let client = reqwest::blocking::Client::new();

    client
        .get("http://localhost:18888/greeting")
        .query(&[("name", "hello world")])
}
