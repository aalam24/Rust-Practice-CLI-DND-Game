use std::vec;
use crate::abilities::AbilityScores;
use crate::spells::SpellSlots;
use std::fmt;


#[derive(Debug, Clone)]
pub enum Class {
    Fighter,
    Barbarian,
    Monk,
    Wizard,
    Sorcerer,
    Cleric
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Class::Fighter => write!(f, "Fighter"),
            Class::Barbarian => write!(f, "Barbarian"),
            Class::Monk => write!(f, "Monk"),
            Class::Wizard => write!(f, "Wizard"),
            Class::Sorcerer => write!(f, "Sorcerer"),
            Class::Cleric => write!(f, "Cleric"),
        }
    }
}

impl Class {
    fn hit_die(&self) -> i32 {
        match self {
            Class::Fighter => 10,
            Class::Barbarian => 12,
            Class::Monk => 8,
            Class::Wizard => 6,
            Class::Sorcerer => 6,
            Class::Cleric => 8,
        }
    }
    pub fn starting_hp(&self, constitution_mod: i32) -> i32 {
        self.hit_die() + constitution_mod;
        let base_hp = self.hit_die() + constitution_mod;
        return base_hp;
    }
    pub fn allocate_spell_slots(&self, level: i32) -> Option<SpellSlots> {
        match self {
            Class::Wizard | Class::Sorcerer | Class::Cleric => Some(SpellSlots::full_caster(level)),
            _ => None,
        }
    }
    /*
    pub fn starting_equipment(&self) -> vec::Vec<String> {
        match self {
            Class::Fighter => <vec!["Longsword".to_string(), "Shield".to_string(), "Chain Mail".to_string()]>,
            Class::Barbarian => <vec!["Greataxe".to_string(),]>,
            Class::Monk => <vec!["Quarterstaff".to_string(),]>,
            Class::Wizard => <vec!["Spellbook".to_string(), "Quarterstaff".to_string()]>,
            Class::Sorcerer => <vec!["Quarterstaff".to_string(),]>,
            Class::Cleric => <vec!["Mace".to_string(), "Shield".to_string(), "Scale Mail".to_string()]>,
        }
    */
    }
    