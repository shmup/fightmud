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

#[test]
fn lowercase_name_works() {
    let mut jared = Entity { id: 0, name: "Jared".to_string() };
    assert_eq!("jared", jared.lowercase());
}

