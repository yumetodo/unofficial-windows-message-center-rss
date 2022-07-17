use super::article::Article;

use scraper::ElementRef;
use scraper::Html;
use scraper::Selector;

#[derive(Debug)]
pub struct Parser {
    base_url: String,
    selector_table_line: Selector,
    selector_article_head: Selector,
    selector_article_id: Selector,
    selector_b: Selector,
    selector_article_body: Selector,
}
impl Parser {
    pub fn new(base_url: &str) -> Self {
        Parser {
            base_url: base_url.to_string(),
            selector_table_line: Selector::parse(r##"#recent-announcements + table tr"##).unwrap(),
            selector_article_head: Selector::parse(r#"td[id] > a[data-linktype]"#).unwrap(),
            selector_article_id: Selector::parse("td[id").unwrap(),
            selector_b: Selector::parse("b").unwrap(),
            selector_article_body: Selector::parse("td[id] > div").unwrap(),
        }
    }
    fn parse_url<'a>(&self, title_element: ElementRef<'a>) -> Option<String> {
        let v = title_element.value();
        match v.attr("data-linktype")? {
            "external" => Some(v.attr("href")?.to_string()),
            "absolute-path" => Some(self.base_url.clone() + v.attr("href")?),
            _ => None,
        }
    }
    fn parse_line<'a>(&self, tr: ElementRef<'a>) -> Option<Article> {
        let title_element = tr.select(&self.selector_article_head).next()?;
        let url = self.parse_url(title_element)?;
        let title: String = title_element
            .select(&self.selector_b)
            .next()?
            .text()
            .collect();
        let id = tr
            .select(&self.selector_article_id)
            .next()?
            .value()
            .attr("id")?
            .to_string();
        let body = tr
            .select(&self.selector_article_body)
            .map(|d: ElementRef| d.html())
            .reduce(|ret, next| ret + &next)?;
        Some(Article::new(id, url, title, body))
    }
    pub fn parse(&self, doc: &str) -> Vec<Article> {
        let document = Html::parse_document(&doc);
        let ret = document
            .select(&self.selector_table_line)
            .filter_map(|tr| self.parse_line(tr))
            .collect();
        ret
    }
}
