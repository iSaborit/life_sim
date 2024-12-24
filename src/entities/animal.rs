#[derive(Clone, Debug)]
pub struct Animal {
    age: u32,
    energy: u32,
    pub position: Position,
    power: u32,
}

#[derive(Clone, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Animal {
    pub fn new(position: Position) -> Animal {
        let animal: Animal = Animal {
            age: 0,
            energy: 100,
            position,
            power: 0,
        };
        animal
    }

    pub fn actually_move_animal(&mut self, x: i32, y: i32) {
        let new_posiiton: Position = Position::new(x, y);
        self.position = new_posiiton;
    }
}

impl Position {
    pub fn new(i: i32, j: i32) -> Position {
        let position: Position = Position { x: i, y: j };
        position
    }

    pub fn get(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}
