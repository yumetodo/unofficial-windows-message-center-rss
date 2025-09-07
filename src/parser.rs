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

        // NaiveDateTimeをパース
        let naive_dt =
            chrono::naive::NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%d %H:%M").ok()?;

        // タイムゾーンを付与してDateTime<FixedOffset>に変換
        let dt = Los_Angeles.from_local_datetime(&naive_dt).single()?;
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
    use regex::Regex;
    const HTML_OF_ABSOLUTE_PATH: &str = r##"
<!DOCTYPE html>
<html>
<body>
<h2 id="recent-announcements">Recent announcements</h2>
<table border ='0'>
<tr><td id="3112">
<a
    href="/en-us/lifecycle/announcements/windows-10-21h2-end-of-servicing"
    target="_blank"
    data-linktype="absolute-path"
    ><b
        >Reminder: End of servicing for Windows 10, version
        21H2 Home, Pro, Pro Education, and Pro for
        Workstations</b
    ></a
></td><td class='has-no-wrap'>2022-05-24 <br>14:00 PT</td></tr>
</table>
</body>
</html>"##;
    const HTML_OF_EXTERNAL: &str = r##"
<!DOCTYPE html>
<html>
<body>
<h2 id="recent-announcements">Recent announcements</h2>
<table border ='0'>
<tr><td id="3110">
<a
    href="https://support.microsoft.com/help/5027231"
    target="_blank"
    data-linktype="external"
    ><b
        >Take action: June 2023 security update is now available</b
    ></a
><a
    class="docon docon-link heading-anchor"
    aria-labelledby="3110"
    href="#3110"
    data-linktype="self-bookmark"
></a
><br />
<div>
    The June 2023 security update release is now available
    for Windows 11 and all supported versions of Windows
    10. We recommend that you install these updates
    promptly. For more information about the contents of
    this update, see the release notes, which are easily
    accessible from the&nbsp;<a
        href="https://support.microsoft.com/help/5027231"
        rel="noopener noreferrer"
        target="_blank"
        data-linktype="external"
        >Windows 11</a
    >&nbsp;and&nbsp;<a
        href="https://support.microsoft.com/help/5027215"
        rel="noopener noreferrer"
        target="_blank"
        data-linktype="external"
        >Windows 10</a
    >&nbsp;update history pages. For instructions on how
    to install this update on your home device, check
    the&nbsp;<a
        href="https://support.microsoft.com/windows/update-windows-3c5ae7fc-9fb6-9af1-1984-b5e0412c556a#WindowsVersion=Windows_11"
        rel="noopener noreferrer"
        target="_blank"
        data-linktype="external"
        >Update Windows</a
    >&nbsp;article. To learn more about the different
    types of monthly quality updates, see
    <a
        href="https://aka.ms/Windows/MonthlyUpdates"
        rel="noopener noreferrer"
        target="_blank"
        data-linktype="external"
        >Windows monthly updates explained</a
    >. To be informed about the latest updates and
    releases, follow us on Twitter <a
        href="https://twitter.com/windowsupdate"
        rel="noopener noreferrer"
        target="_blank"
        data-linktype="external"
        ><u>@WindowsUpdate</u></a
    >.
</div>
<div>&nbsp;&nbsp;</div>
<div>
    <strong
        >Highlights for the Windows 11 update:&nbsp;</strong
    >
</div>
<ul>
    <li>
        This update addresses security issues for your
        Windows operating system.&nbsp;&nbsp;&nbsp;
    </li>
    <li>
        This update addresses a known issue that affects
        32-bit apps that are&nbsp;<a
            href="/en-us/windows/win32/memory/memory-limits-for-windows-releases#memory-and-address-space-limits"
            rel="noopener noreferrer"
            target="_blank"
            data-linktype="absolute-path"
            >large address aware</a
        >&nbsp;and use the&nbsp;<a
            href="/en-us/windows/win32/api/winbase/nf-winbase-copyfile"
            rel="noopener noreferrer"
            target="_blank"
            data-linktype="absolute-path"
            >CopyFile API</a
        >. You might have issues when you save, copy, or
        attach files. If you use some commercial or
        enterprise security software that uses extended file
        attributes, this issue will likely affect you. For
        Microsoft Office apps, this issue only affects the
        32-bit versions. You might receive the error,
        "Document not saved."&nbsp;
    </li>
</ul>
<div></div>
<div>
    Short on time? Watch our short
    <a
        href="https://youtu.be/2CPFOqAoG4o"
        rel="noopener noreferrer"
        target="_blank"
        data-linktype="external"
        >Windows 11 update</a
    >
    release notes video for this month's tips.
</div>
</td><td class='has-no-wrap'>2022-05-24 <br>14:00 PT</td></tr>
</table>
</body>
</html>"##;
    const HTML_OF_SELF_BOOKMARK: &str = r##"
<!DOCTYPE html>
<html>
<body>
<h2 id="recent-announcements">Recent announcements</h2>
<table border ='0'>
<tr><td id="3113">
<b
    >Reminder: Security hardening changes for Netlogon and Kerberos coming in June and July 2023</b
><a
    class="docon docon-link heading-anchor"
    aria-labelledby="3113"
    href="#3113"
    data-linktype="self-bookmark"
></a
></td><td class='has-no-wrap'>2022-05-24 <br>14:00 PT</td></tr>
</table>
</body>
</html>"##;
    #[test]
    fn parse_date() {
        let doc = Html::parse_document(HTML_OF_EXTERNAL);
        let selector = Selector::parse("td[id] + td").unwrap();
        let elem = doc.select(&selector).next().unwrap();
        let parsed = Parser::new("", "").parse_date(elem);
        assert_eq!(parsed, Some("2022-05-24T21:00:00Z".to_string()));
    }
    #[test]
    fn parse_url_when_absolute_path() {
        let doc = Html::parse_document(HTML_OF_ABSOLUTE_PATH);
        let p = Parser::new("", "https://learn.microsoft.com");
        let title_element = doc
            .select(&p.selector_table_line)
            .next()
            .unwrap()
            .select(&p.selector_article_head)
            .next()
            .unwrap();
        let ret = p.parse_url(title_element);
        assert!(ret.is_some());
        let (url, rel) = ret.unwrap();
        assert_eq!(url, "https://learn.microsoft.com/en-us/lifecycle/announcements/windows-10-21h2-end-of-servicing");
        assert_eq!(rel, "alternate");
    }

    #[test]
    fn parse_url_when_external() {
        let doc = Html::parse_document(HTML_OF_EXTERNAL);
        let p = Parser::new("", "");
        let title_element = doc
            .select(&p.selector_table_line)
            .next()
            .unwrap()
            .select(&p.selector_article_head)
            .next()
            .unwrap();
        let ret = p.parse_url(title_element);
        assert!(ret.is_some());
        let (url, rel) = ret.unwrap();
        assert_eq!(url, "https://support.microsoft.com/help/5027231");
        assert_eq!(rel, "alternate");
    }
    #[test]
    fn parse_url_when_self_bookmark() {
        let doc = Html::parse_document(HTML_OF_SELF_BOOKMARK);
        let p = Parser::new(
            "https://yumetodo.github.io/unofficial-windows-message-center-rss/feed/atom10.xml",
            "",
        );
        let title_element = doc
            .select(&p.selector_table_line)
            .next()
            .unwrap()
            .select(&p.selector_article_head)
            .next()
            .unwrap();
        let ret = p.parse_url(title_element);
        assert!(ret.is_some());
        let (url, rel) = ret.unwrap();
        assert_eq!(
            url,
            "https://yumetodo.github.io/unofficial-windows-message-center-rss/feed/atom10.xml"
        );
        assert_eq!(rel, "self");
    }
    #[test]
    fn parse_title_when_alternate() {
        let doc = Html::parse_document(HTML_OF_EXTERNAL);
        let p = Parser::new("", "");
        let tr: ElementRef<'_> = doc.select(&p.selector_table_line).next().unwrap();
        let title_element = tr.select(&p.selector_article_head).next().unwrap();
        let ret = p.parse_title(tr, title_element, "alternate");
        assert!(ret.is_some());
        let title = ret.unwrap();
        assert_eq!(
            title,
            "Take action: June 2023 security update is now available"
        );
    }
    #[test]
    fn parse_title_when_self() {
        let doc = Html::parse_document(HTML_OF_SELF_BOOKMARK);
        let p = Parser::new("", "");
        let tr: ElementRef<'_> = doc.select(&p.selector_table_line).next().unwrap();
        let title_element = tr.select(&p.selector_article_head).next().unwrap();
        let ret = p.parse_title(tr, title_element, "self");
        assert!(ret.is_some());
        let title = ret.unwrap();
        assert_eq!(
            title,
            "Reminder: Security hardening changes for Netlogon and Kerberos coming in June and July 2023"
        );
    }
    #[test]
    fn parse() {
        let p = Parser::new("", "");
        let ret = p.parse(HTML_OF_EXTERNAL);
        assert_eq!(ret.len(), 1);
        let article = &ret[0];
        assert_eq!(article.id, "3110");
        assert_eq!(article.url, "https://support.microsoft.com/help/5027231");
        assert_eq!(article.rel, "alternate");
        assert_eq!(
            article.title,
            "Take action: June 2023 security update is now available"
        );
        assert_eq!(article.date, "2022-05-24T21:00:00Z");
        let count_div = Regex::new(r"</?div>").unwrap();
        assert_eq!(count_div.find_iter(&article.body).count(), 10);
        assert!(article.body.contains(
            r"The June 2023 security update release is now available
    for Windows 11 and all supported versions of Windows
    10. We recommend that you install these updates
    promptly. For more information about the contents of
    this update, see the release notes, which are easily
    accessible from the"
        ));
        assert!(article.body.contains(
            r"article. To learn more about the different
    types of monthly quality updates, see"
        ));
        assert!(article.body.contains(
            r"To be informed about the latest updates and
    releases, follow us on Twitter"
        ));
        assert!(article
            .body
            .contains("Highlights for the Windows 11 update"));
        assert!(article.body.contains("Short on time? Watch our short"));
        assert!(article
            .body
            .contains("release notes video for this month's tips."));
    }
}
