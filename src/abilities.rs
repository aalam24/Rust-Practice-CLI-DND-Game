pub struct AbilityScores {
    // DND 5E Ability Scores, range from 1 to 30
    pub strength: i32,
    pub dexterity: i32,
    pub constitution: i32,
    pub intelligence: i32,
    pub wisdom: i32,
    pub charisma: i32,
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

