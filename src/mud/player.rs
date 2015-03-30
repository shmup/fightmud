pub struct Point {
    pub x: i32,
    pub y: i32
}

pub struct Player {
    pub name: String,
    health: i32,
    attack: i32,
    pub position: Point
}

impl Player {
    pub fn new() -> Player {
        Player {
            name: "Jared".to_string(),
            health: 10,
            attack: 10,
            position: Point { x: 1, y: 0 }
        }
    }

    pub fn attack(&self) {
        println!("{} attacks for {} damage", self.name, self.attack);
    }
}
