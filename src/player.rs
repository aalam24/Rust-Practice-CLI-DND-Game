use crate::races::Race;
use crate::classes::Class;
use crate::abilities::AbilityScores;
use crate::spells::SpellSlots;

pub struct Player {
    pub name: String,
    pub current_health: i32,
    pub max_health: i32,
    pub race: Race,
    pub class: Class,
    pub ability_scores: AbilityScores,
    pub spell_slots: Option<SpellSlots>,
}