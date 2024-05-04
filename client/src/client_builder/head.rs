use reqwest::blocking::RequestBuilder;

pub fn head() -> RequestBuilder {
    let client = reqwest::blocking::Client::new();

    client
        .head("http://localhost:18888/greeting")
        .query(&[("name", "hello world")])
}
