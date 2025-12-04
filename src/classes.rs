use crate::abilities::AbilityScores;


#[derive(Debug, Clone)]
pub enum Class {
    Fighter,
    Barbarian,
    Monk,
    Wizard,
    Sorcerer,
    Cleric
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
    fn starting_hp(&self, constitution_mod: i32) -> i32 {
        self.hit_die() + constitution_mod
        let base_hp = self.hit_die() + constitution_mod;
    }
    fn is_spellcaster(&self) -> bool {
        match self {
            Class::Wizard | Class::Sorcerer | Class::Cleric => true,
            _ => false,
        }
    }git 
}