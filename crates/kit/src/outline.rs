#[derive(Clone, Debug)]
pub struct OutlineItem {
    pub title: String,
    pub page: u32,
    pub children: Vec<OutlineItem>,
}

#[derive(Clone, Debug, Default)]
pub struct Outline {
    items: Vec<OutlineItem>,
}

impl Outline {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
        }
    }

    pub fn add(&mut self, item: OutlineItem) {
        self.items.push(item);
    }

    pub fn items(&self) -> &[OutlineItem] {
        &self.items
    }
}
