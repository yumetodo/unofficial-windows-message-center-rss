#[derive(Debug)]
pub struct Article {
    id: String,
    url: String,
    title: String,
    date: String,
    body: String,
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
