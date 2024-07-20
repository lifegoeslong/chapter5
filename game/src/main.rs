#[derive(Debug)]
struct Character {
    name: String,
    health: i32,
    attack: i32,
    defense: i32,
}

#[derive(Debug)]
struct Item {
    name: String,
    health_boost: i32,
    attack_boost: i32,
    defense_boost: i32,
}

impl Character {
    fn new(name: &str, health: i32, attack: i32, defense: i32) -> Character {
        Character {
            name: name.to_string(),
            health,
            attack,
            defense,
        }
    }

    fn attack(&self, other: &mut Character) {
        let damage = self.attack - other.defense;
        if damage > 0 {
            other.health -= damage;
            println!(
                "{} attacks {} for {} damage!",
                self.name, other.name, damage
            );
        } else {
            println!("{} attacks {}, but it's not effective!", self.name, other.name);
        }
    }

    fn use_item(&mut self, item: &Item) {
        self.health += item.health_boost;
        self.attack += item.attack_boost;
        self.defense += item.defense_boost;
        println!(
            "{} uses {}. Health: {}, Attack: {}, Defense: {}",
            self.name, item.name, self.health, self.attack, self.defense
        );
    }

    fn is_alive(&self) -> bool {
        self.health > 0
    }
}

fn main() {
    let mut hero = Character::new("Hero", 100, 15, 5);
    let mut goblin = Character::new("Goblin", 80, 10, 3);

    let potion = Item {
        name: String::from("Health Potion"),
        health_boost: 20,
        attack_boost: 0,
        defense_boost: 0,
    };

    let sword = Item {
        name: String::from("Sword of Might"),
        health_boost: 0,
        attack_boost: 10,
        defense_boost: 0,
    };

    // Hero uses items
    hero.use_item(&potion);
    hero.use_item(&sword);

    // Battle loop
    while hero.is_alive() && goblin.is_alive() {
        hero.attack(&mut goblin);
        if goblin.is_alive() {
            goblin.attack(&mut hero);
        }
    }

    if hero.is_alive() {
        println!("The hero has defeated the goblin!");
    } else {
        println!("The goblin has defeated the hero!");
    }
}

