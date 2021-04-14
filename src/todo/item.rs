pub struct Item {
    pub id: u32,
    pub desc: String,
}

impl Item {
    pub fn new(id: u32, desc: String) -> Self {
        Self {
            id,
            desc,
        }
    }
}
