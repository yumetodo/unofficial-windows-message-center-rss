mod article;
mod feed;
mod parser;
use article::Article;
use chrono::Utc;
use feed::*;
use parser::Parser;
use std::io::Write;

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
    let entries = articles
        .into_iter()
        .map(|a: Article| {
            Entry::new(format!("https://docs.microsoft.com/en-us/windows/release-health/windows-message-center#{}", a.id), a.title, a.date)
                .link(Link::new().href(a.url).rel("alternate"))
                .content(a.body)
        })
        .collect::<Vec<Entry>>();
    let feed = Feed::new(
        "https://github.com/yumetodo/unofficial-windows-message-center-rss".into(),
        "Windows message center - Recent announcements".into(),
        Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
    )
    .author(vec![Person::new("direek"), Person::new("Microsoft")])
    .link(vec![
        Link::new()
            .href("https://docs.microsoft.com/en-us/windows/release-health/windows-message-center")
            .type_("text/html"),
    ])
    .entry(entries);
    std::io::stdout()
        .write_all(feed.to_xml().as_bytes())
        .unwrap();
}
