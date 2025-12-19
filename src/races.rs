use crate::abilities::AbilityScores;
use std::io::{self, Write};
use std::fmt;

pub enum Race {
    Human,
    Elf,
    Dwarf,
    Orc,
    Tiefling,
    Gnome
}

impl fmt::Display for Race {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Race::Human => write!(f, "Human"),
            Race::Elf => write!(f, "Elf"),
            Race::Dwarf => write!(f, "Dwarf"),
            Race::Orc => write!(f, "Orc"),
            Race::Tiefling => write!(f, "Tiefling"),
            Race::Gnome => write!(f, "Gnome"),
        }
    }
}

impl Race {
    // Apply racial ability score increases
    pub fn apply_racial_bonuses(&self, ability_scores: &mut AbilityScores, choices: (String, String)) {
        let (plus2, plus1) = choices;

        match plus2.as_str() {
            "strength" => ability_scores.strength += 2,
            "dexterity" => ability_scores.dexterity += 2,
            "constitution" => ability_scores.constitution += 2,
            "intelligence" => ability_scores.intelligence += 2,
            "wisdom" => ability_scores.wisdom += 2,
            "charisma" => ability_scores.charisma += 2,
            _ => {}
        }
        match plus1.as_str() {
            "strength" => ability_scores.strength += 1,
            "dexterity" => ability_scores.dexterity += 1,
            "constitution" => ability_scores.constitution += 1,
            "intelligence" => ability_scores.intelligence += 1,
            "wisdom" => ability_scores.wisdom += 1,
            "charisma" => ability_scores.charisma += 1,
            _ => {}
        }
    }  
    pub fn choose_ability_score_increases() -> (String, String) {
        println!("Choose two ability scores to increase");
        let plus2 = loop {
            print!("Choose ability to increase by +2: ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim().to_lowercase();

            if  ["str", "dex", "con", "int", "wis", "cha"].contains(&input.as_str()) {
                break input;
            } else {
                println!("Invalid choice. Please choose from str, dex, con, int, wis, cha.");
            }
        };
        let plus1 = loop {
            print!("Choose ability to increase by +1: ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim().to_lowercase();

            if  ["str", "dex", "con", "int", "wis", "cha"].contains(&input.as_str()) {
                break input;
            } else {
                println!("Invalid choice. Please choose from str, dex, con, int, wis, cha.");
            }
        };
        println!("You chose to increase {} by +2 and {} by +1", plus2, plus1);
        (plus2, plus1)
    }
    
    //TODO: Add racial traits and features like Tiefling's fire resistance, Darkvision, etc.

}