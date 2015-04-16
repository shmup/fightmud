use std::ascii::AsciiExt;

struct Entity {
    pub id: u32,
    pub name: String
}

impl Entity {
    pub fn lowercase(&mut self) -> String {
        self.name.to_ascii_lowercase() 
    }
}
