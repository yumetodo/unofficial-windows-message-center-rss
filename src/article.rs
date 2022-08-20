#[derive(Debug)]
pub struct Article {
    pub id: String,
    pub url: String,
    pub title: String,
    pub date: String,
    pub body: String,
}
impl Article {
    pub fn new(id: String, url: String, title: String, date: String, body: String) -> Self {
        Article {
            id,
            url,
            title,
            date,
            body,
        }
    }
}
