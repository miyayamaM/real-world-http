use reqwest::Error;

fn main() -> Result<(), Error> {
    let body = reqwest::blocking::get("http://localhost:18888/greeting")?.text()?;
    println!("{}", body);
    Ok(())
}
