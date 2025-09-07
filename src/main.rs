mod article;
mod feed;
mod parser;
use article::Article;
use chrono::Utc;
use feed::*;
use parser::Parser;
use std::env;
use std::fs;
use std::io::Write;

fn read_from_web() -> reqwest::Result<String> {
    let client = reqwest::blocking::Client::builder()
        .user_agent("Mozilla/5.0 reqwest/0.12.23 https://github.com/yumetodo/unofficial-windows-message-center-rss")
        .build()?;
    let doc = client
        .get("https://learn.microsoft.com/en-us/windows/release-health/windows-message-center")
        .send()?
        .text()?;
    Ok(doc)
}

fn read_html(path: Option<String>) -> String {
    match path {
        Some(p) => fs::read_to_string(p).unwrap(),
        None => read_from_web().unwrap(),
    }
}

struct Options {
    self_uri: String,
    path: Option<String>,
}
impl Options {
    fn new(args: Vec<String>) -> Self {
        if args.len() != 2 && args.len() != 3 {
            panic!("{} SELF_URI [path]", args[0]);
        }
        Self {
            self_uri: args[1].clone(),
            path: if args.len() == 3 {
                Some(args[2].clone())
            } else {
                None
            },
        }
    }
}

fn main() {
    let options = Options::new(env::args().collect::<Vec<String>>());
    let doc = read_html(options.path);
    let articles = Parser::new(&options.self_uri, "https://learn.microsoft.com").parse(&doc);
    let entries = articles
        .into_iter()
        .map(|a: Article| {
            Entry::new(format!("https://learn.microsoft.com/en-us/windows/release-health/windows-message-center#{}", a.id), a.title.into(), a.date)
                .link(Link::new().href(a.url).rel(a.rel))
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
            .href("https://learn.microsoft.com/en-us/windows/release-health/windows-message-center")
            .type_("text/html"),
        Link::new()
            .href(&options.self_uri)
            .type_("application/atom+xml")
            .rel("self"),
    ])
    .entry(entries);
    std::io::stdout()
        .write_all(feed.to_xml().as_bytes())
        .unwrap();
}
