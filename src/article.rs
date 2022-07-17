#[derive(Debug)]
pub struct Article {
    id: String,
    url: String,
    title: String,
    body: String,
}
impl Article {
    pub fn new(id: String, url: String, title: String, body: String) -> Self {
        Article {
            id,
            url,
            title,
            body,
        }
    }
}
