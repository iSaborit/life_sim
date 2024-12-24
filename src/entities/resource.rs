enum ResourceType {
    Plant,
    Insect,
    DeadAnimal,
}

struct Resource {
    res_type: ResourceType,
    poisonous: bool,
}

impl Resource {
    pub fn new() -> Resource {
        let rng: i32 = rand::thread_rng().gen_range(0..=2);
        let res_type = match rng {
            0 => ResourceType::Plant,
            1 => ResourceType::Insect,
            2 => ResourceType::DeadAnimal,
        };
        let poisonous = rand::thread_rng().gen_bool(0..=20) % 20 == 0;
    }
}
