#[derive(Debug)]
pub struct Article {
    pub id: String,
    pub url: String,
    pub rel: &'static str,
    pub title: String,
    pub date: String,
    pub body: String,
}
impl Article {
    pub fn new(
        id: String,
        url: String,
        rel: &'static str,
        title: String,
        date: String,
        body: String,
    ) -> Self {
        Article {
            id,
            url,
            rel,
            title,
            date,
            body,
        }
    }
}
