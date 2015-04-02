pub struct Point {
    pub x: i32,
    pub y: i32
}

pub struct Player {
    pub name: String,
    health: i32,
    attack: i32,
    pub inventory: Inventory,
    pub position: Point
}

impl Player {
    pub fn new() -> Player {

        let mut items: Vec<Item> = Vec::new();

        let inventory = Inventory {
            items: items,
            uses: 10
        };

        Player {
            name: "Jared".to_string(),
            health: 10,
            attack: 10,
            inventory: inventory,
            position: Point { x: 1, y: 0 }
        }
    }

    pub fn attack(&self) {
        println!("{} attacks for {} damage", self.name, self.attack);
    }
}

pub struct Inventory {
    pub items: Vec<Item>,
    uses: i32
}

pub struct Item {
    name: String
}


