#[derive(Debug, Clone)]
pub struct Tag {
    tag_name: String,
}

impl Tag {
    pub fn new(tag: String) -> Self {
        Tag {
            tag_name: tag
        }
    }

    pub fn tag(&self) -> &String {
        &self.tag_name
    }
}