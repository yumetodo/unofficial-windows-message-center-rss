mod article;
mod feed;
mod parser;
use article::Article;
use chrono::Utc;
use feed::*;
use parser::Parser;
use std::io::Write;
use std::env;
use std::fs;

fn read_from_web() -> reqwest::Result<String> {
    let client = reqwest::blocking::Client::builder()
        .user_agent("Mozilla/5.0 reqwest/0.11 https://github.com/yumetodo/unofficial-windows-message-center-rss")
        .build()?;
    let doc = client
        .get("https://docs.microsoft.com/en-us/windows/release-health/windows-message-center")
        .send()?
        .text()?;
    Ok(doc)
}

fn read_html(args: &Vec<String>) -> String {
    if args.len() == 2 {
        fs::read_to_string(&args[1]).unwrap()
    } else {
        read_from_web().unwrap()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let doc = read_html(&args);
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
        Link::new()
            .href("https://yumetodo.github.io/unofficial-windows-message-center-rss/feed/atom10.xml")
            .type_("application/atom+xml")
            .rel("self"),
    ])
    .entry(entries);
    std::io::stdout()
        .write_all(feed.to_xml().as_bytes())
        .unwrap();
}
