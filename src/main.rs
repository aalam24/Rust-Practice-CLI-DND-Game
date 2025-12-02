mod player;
mod classes;
mod races;
mod abilities;
mod spells;

use crate::player::Player;
use crate::races::Race;
use crate::classes::Class;

enum GameState {
    MainMenu,
    CharacterCreation,
    Exploring,
    InCombat,
    Paused,
    GameOver
}

fn create_character() -> Player {
    GameState::CharacterCreation;
    //1. Name
    println!("Enter your character's name:");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim().to_string();
    //2. Choose race
    let race = loop {
        println!("Choose {}'s race:", name);
        println!("1. Human, 2. Elf, 3. Dwarf, 4. Orc, 5. Tiefling, 6. Gnome");
        let mut race_input = String::new();
        std::io::stdin().read_line(&mut race_input).expect("Failed to read line");

        match race_input.trim().parse::<u32>() {
            Ok(1) => break Race::Human,
            Ok(2) => break Race::Elf,
            Ok(3) => break Race::Dwarf,
            Ok(4) => break Race::Orc,
            Ok(5) => break Race::Tiefling,
            Ok(6) => break Race::Gnome,
            _ => {
                println!("Invalid choice, enter a number between 1 and 6.")
            }
        }
    };
    //3. Choose class
    let class = loop {
        println!("Choose {}'s class:", name);
        println!("1. Fighter, 2. Barbarian, 3. Monk, 4. Wizard, 5. Sorcerer, 6. Cleric");
        let mut class_input = String::new();
        std::io::stdin().read_line(&mut class_input).expect("Failed to read line");

        match class_input.trim().parse::<u32>() {
            Ok(1) => break Class::Fighter,
            Ok(2) => break Class::Barbarian,
            Ok(3) => break Class::Monk,
            Ok(4) => break Class::Wizard,
            Ok(5) => break Class::Sorcerer,
            Ok(6) => break Class::Cleric,
            _ => {
                println!("Invalid choice, enter a number between 1 and 6.")
            }
        }
    };
    //4. Allocate ability scores

    //4. Allocate ability scores
    // TODO: Implement ability score allocation
    let ability_scores = crate::abilities::AbilityScores {
        strength: 10,
        dexterity: 10,
        constitution: 10,
        intelligence: 10,
        wisdom: 10,
        charisma: 10,
    };

    //5. Create character
    Player {
        name,
        current_health: 10,
        max_health: 10,
        race,
        class,
        ability_scores,
        spell_slots: None,
    }
}

fn main() {

}