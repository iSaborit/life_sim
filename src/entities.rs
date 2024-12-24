use animal::Position;

pub mod animal;

#[derive(Clone, Debug)]
pub enum Entity {
    Animal,
    Resource,
}

#[derive(Clone, Debug)]
pub struct Simulator {
    pub world: Vec<Vec<Option<Entity>>>,
    animals: Vec<animal::Animal>,
    dimension: i32,
}

impl Simulator {
    pub fn new(
        world: Vec<Vec<Option<Entity>>>,
        animals: Vec<animal::Animal>,
        dimension: i32,
    ) -> Simulator {
        Simulator {
            world,
            animals,
            dimension,
        }
    }

    pub fn determine_movement(&mut self) {
        let world_snapshot = self.world.clone(); // Hacemos una copia del mundo actual.
        let mut animals_snapshot = self.animals.clone();
        for animal in animals_snapshot.iter_mut() {
            let x = animal.position.x;
            let y = animal.position.y;

            if Simulator::check_if_resources_with_snapshot(&world_snapshot, x + 1, y) {
                self.move_animal(x, y, x + 1, y);
                animal.position = Position::new(x + 1, y);
            } else if Simulator::check_if_resources_with_snapshot(&world_snapshot, x, y + 1) {
                self.move_animal(x, y, x, y + 1);
                animal.position = Position::new(x, y + 1);
            } else if Simulator::check_if_resources_with_snapshot(&world_snapshot, x - 1, y) {
                self.move_animal(x, y, x - 1, y);
                animal.position = Position::new(x - 1, y);
            } else if Simulator::check_if_resources_with_snapshot(&world_snapshot, x, y - 1) {
                self.move_animal(x, y, x, y - 1);
                animal.position = Position::new(x, y - 1);
            } else {
                // Lógica alternativa si no se puede mover.
            }
        }
        self.animals = animals_snapshot;
    }

    fn move_animal(&mut self, x: i32, y: i32, nx: i32, ny: i32) {
        self.world[x as usize][y as usize] = None;
        self.world[nx as usize][ny as usize] = Some(Entity::Animal);
    }

    fn check_if_resources_with_snapshot(
        snapshot: &Vec<Vec<Option<Entity>>>,
        x: i32,
        y: i32,
    ) -> bool {
        if x < 0 || y < 0 {
            return false;
        }

        let x = x as usize;
        let y = y as usize;

        if x < snapshot.len() && y < snapshot[x].len() {
            matches!(snapshot[x][y], Some(Entity::Resource))
        } else {
            false
        }
    }
}
