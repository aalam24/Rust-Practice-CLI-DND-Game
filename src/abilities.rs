struct AbilityScores {
    // DND 5E Ability Scores, range from 1 to 30
    strength: i32,
    dexterity: i32,
    constitution: i32,
    intelligence: i32,
    wisdom: i32,
    charisma: i32,
}

impl AbilityScores {
    // Calculate the modifiers for each ability score
    pub fn str_mod(&self) -> i32 {
        (self.strength - 10) / 2
    }
    pub fn dex_mod(&self) -> i32 {
        (self.dexterity - 10) / 2
    }
    pub fn con_mod(&self) -> i32 {
        (self.constitution - 10) / 2
    }
    pub fn int_mod(&self) -> i32 {
        (self.intelligence - 10) / 2
    }
    pub fn wis_mod(&self) -> i32 {
        (self.wisdom - 10) / 2
    }
    pub fn cha_mod(&self) -> i32 {
        (self.charisma - 10) / 2
    }
}

