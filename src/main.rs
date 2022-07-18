mod article;
mod parser;
mod feed;
use parser::Parser;

fn main() {
    let client = reqwest::blocking::Client::builder()
        .user_agent("Mozilla/5.0 reqwest/0.11 https://github.com/yumetodo/unofficial-windows-message-center-rss")
        .build()
        .unwrap();
    let doc = client
        .get("https://docs.microsoft.com/en-us/windows/release-health/windows-message-center")
        .send()
        .unwrap()
        .text()
        .unwrap();
    let articles = Parser::new("https://docs.microsoft.com").parse(&doc);
    println!("{:#?}", articles);
}
