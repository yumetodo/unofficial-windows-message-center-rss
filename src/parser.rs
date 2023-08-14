use super::article::Article;

use chrono::TimeZone;
use chrono_tz::America::Los_Angeles;
use chrono_tz::UTC;
use scraper::ElementRef;
use scraper::Html;
use scraper::Selector;

#[derive(Debug)]
pub struct Parser {
    self_uri: String,
    base_url: String,
    selector_table_line: Selector,
    selector_article_head: Selector,
    selector_article_id: Selector,
    selector_b_for_alternate: Selector,
    selector_b_for_self: Selector,
    selector_article_body: Selector,
    selector_article_date: Selector,
}
impl Parser {
    pub fn new(self_uri: &str, base_url: &str) -> Self {
        Parser {
            self_uri: self_uri.to_string(),
            base_url: base_url.to_string(),
            selector_table_line: Selector::parse(r##"#recent-announcements + table tr"##).unwrap(),
            selector_article_head: Selector::parse(r#"td[id] > a[data-linktype]"#).unwrap(),
            selector_article_id: Selector::parse("td[id").unwrap(),
            selector_b_for_alternate: Selector::parse("b").unwrap(),
            selector_b_for_self: Selector::parse("td[id] > b").unwrap(),
            selector_article_body: Selector::parse("td[id] > div").unwrap(),
            selector_article_date: Selector::parse("td[id] + td").unwrap(),
        }
    }
    fn parse_url<'a>(&self, title_element: ElementRef<'a>) -> Option<(String, &'static str)> {
        let v = title_element.value();
        match v.attr("data-linktype")? {
            "external" => Some((v.attr("href")?.to_string(), "alternate")),
            "absolute-path" => Some((self.base_url.clone() + v.attr("href")?, "alternate")),
            "self-bookmark" => Some((self.self_uri.clone(), "self")),
            _ => None,
        }
    }
    fn parse_date<'a>(&self, date_element: ElementRef<'a>) -> Option<String> {
        let mut date_str: String = date_element.text().collect();
        date_str.truncate(date_str.find("PT")?);
        if let Some(end_pos) = date_str.rfind(char::is_whitespace) {
            date_str.truncate(end_pos);
        }
        let dt = Los_Angeles
            .datetime_from_str(&date_str, "%Y-%m-%d %H:%M")
            .unwrap();
        let utc = dt
            .with_timezone(&UTC)
            .format("%Y-%m-%dT%H:%M:%SZ")
            .to_string();
        Some(utc)
    }
    fn parse_title<'a>(
        &self,
        tr: ElementRef<'a>,
        title_element: ElementRef<'a>,
        rel: &'static str,
    ) -> Option<String> {
        let s = match rel {
            "self" => tr.select(&self.selector_b_for_self).next(),
            "alternate" => title_element.select(&self.selector_b_for_alternate).next(),
            _ => None,
        }?;
        Some(s.text().collect())
    }
    fn parse_line<'a>(&self, tr: ElementRef<'a>) -> Option<Article> {
        let title_element = tr.select(&self.selector_article_head).next()?;
        println!("{:?}", title_element.value());
        let (url, rel) = self.parse_url(title_element)?;
        let title = self.parse_title(tr, title_element, rel)?;
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
        let date = tr.select(&self.selector_article_date).next()?;
        let date = self.parse_date(date)?;
        Some(Article::new(id, url, rel, title, date, body))
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn parse_date() {
        let input = r#"
<!DOCTYPE html>
<html>
<body>
<h2 id="recent-announcements">Recent announcements</h2>
<table border ='0'>
<tr><td id='2832'>aaa</td><td class='has-no-wrap'>2022-05-24 <br>14:00 PT</td></tr>
</table>
</body>
</html>"#;
        let doc = Html::parse_document(input);
        let selector = Selector::parse("td[id] + td").unwrap();
        let elem = doc.select(&selector).next().unwrap();
        let parsed = Parser::new("", "").parse_date(elem);
        assert_eq!(parsed, Some("2022-05-24T21:00:00Z".to_string()));
    }
}
