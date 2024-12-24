use std::io;

mod entities;
use entities::animal;

use rand::Rng;

fn setup(dimensions: usize) -> entities::Simulator {
    let mut world: Vec<Vec<Option<entities::Entity>>> = vec![vec![None; dimensions]; dimensions];
    let mut animals: Vec<animal::Animal> = vec![];
    let mut rng = rand::thread_rng();

    for i in 0..dimensions {
        for j in 0..dimensions {
            if rng.gen_range(0..=10) % 10 == 0 {
                if rng.gen_range(0..=1) == 1 {
                    let position = animal::Position::new(i as i32, j as i32);
                    world[i][j] = Some(entities::Entity::Animal);
                    animals.push(animal::Animal::new(position));
                } else {
                    world[i][j] = Some(entities::Entity::Resource)
                }
            }
        }
    }
    entities::Simulator::new(world, animals, dimensions as i32)
}

fn turn(simulation: &mut entities::Simulator) -> entities::Simulator {
    simulation.determine_movement();
    simulation.clone()
}

pub fn run() {
    println!("Dimensions of the world: ");

    let mut string_dim: String = String::new();
    if let Err(error) = io::stdin().read_line(&mut string_dim) {
        eprintln!("{}", error);
        std::process::exit(1);
    }

    let dimensions: i32 = string_dim
        .trim()
        .parse::<i32>()
        .expect("Should been able to parse it.");

    let mut world = setup(dimensions as usize);
    world.world.iter().for_each(|case| {
        case.iter().for_each(|casilla| match casilla {
            Some(e) => match e {
                entities::Entity::Animal => print!(" A "),
                entities::Entity::Resource => print!(" r "),
            },
            None => print!(" - "),
        });
        println!();
    });

    println!("-----------------------");
    world = turn(&mut world);
    world.world.iter().for_each(|case| {
        case.iter().for_each(|casilla| match casilla {
            Some(e) => match e {
                entities::Entity::Animal => print!(" A "),
                entities::Entity::Resource => print!(" r "),
            },
            None => print!(" - "),
        });
        println!();
    });

    println!("Dimensions of the world: {string_dim}");

    todo!();
}
