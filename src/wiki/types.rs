//Wiki
//represents basic information about a wiki page
pub struct Wiki {
    pub title: String,
    pub summary: String,
}

impl Wiki {
    pub fn new(title: String, summary: String) -> Self {
        Wiki { title, summary }
    }
}
