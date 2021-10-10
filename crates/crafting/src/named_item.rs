/// NamedItem
/// Generic structure for items that define Id and Name
pub struct NamedItem {
    pub id: String,
    pub name: String
}

impl NamedItem {
    pub fn new(id: String, name: String) -> NamedItem {
        NamedItem {
            id,
            name
        }
    }
}