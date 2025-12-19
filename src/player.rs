use crate::races::Race;
use crate::classes::Class;
use crate::abilities::AbilityScores;
use crate::spells::SpellSlots;
use macroquad::prelude::*;

pub struct Player {
    pub name: String,
    pub current_health: i32,
    pub max_health: i32,
    pub race: Race,
    pub class: Class,
    pub ability_scores: AbilityScores,
    pub spell_slots: Option<SpellSlots>,
    pub level: i32,
    pub equipment: Vec<String>,
    pub base_armor: i32,
    pub bonus_armor: i32,
    pub has_shield: bool,
}

impl Player {
    pub fn calculate_ac(&self) -> i32 {
        let mut armor_class = self.base_armor + self.bonus_armor;
        if self.has_shield {
            armor_class += 2;
        }
        armor_class
    }

    pub async fn display_character_sheet(&self) {
        loop {
            clear_background(DARKGRAY);
            draw_text(&self.name, 40.0, 50.0,30.0, WHITE);
            draw_text(
            &format!("Level: {} {} {}", self.level, self.race, self.class),
            40.0, 90.0, 24.0, LIGHTGRAY
            );
            draw_text(
            &format!("Health: {}/{}", self.current_health, self.max_health),
            40.0, 130.0, 24.0, LIGHTGRAY
            );
            draw_text(
            &format!("Armor Class: {}", self.calculate_ac()),
            40.0, 165.0, 24.0, SKYBLUE
            );
            draw_text("--- Ability Scores ---", 40.0, 210.0, 22.0, GOLD);

            let y = 240.0;
            let h = 30.0;
            draw_text(&format!("STR: {} ({:+})", self.ability_scores.strength, self.ability_scores.str_mod()), 40.0, y, 22.0, WHITE);
            draw_text(&format!("DEX: {} ({:+})", self.ability_scores.dexterity, self.ability_scores.dex_mod()), 40.0, y + h, 22.0, WHITE);
            draw_text(&format!("CON: {} ({:+})", self.ability_scores.constitution, self.ability_scores.con_mod()), 40.0, y + h*2.0, 22.0, WHITE);
            draw_text(&format!("INT: {} ({:+})", self.ability_scores.intelligence, self.ability_scores.int_mod()), 40.0, y+ h*3.0, 22.0, WHITE);
            draw_text(&format!("WIS: {} ({:+})", self.ability_scores.wisdom, self.ability_scores.wis_mod()), 40.0, y+ h*4.0, 22.0, WHITE);
            draw_text(&format!("CHA: {} ({:+})", self.ability_scores.charisma, self.ability_scores.cha_mod()), 40.0, y+ h*5.0, 22.0, WHITE);
        
            if let Some(ref slots) = self.spell_slots {
                draw_text("--- Spell Slots ---", 40.0, y + h*7.0, 22.0, PURPLE);
                draw_text(
                &format!("1st: {}/{}  2nd: {}/{}  3rd: {}/{}",
                slots.level_1.0, slots.level_1.1,
                slots.level_2.0, slots.level_2.1,
                slots.level_3.0, slots.level_3.1),
                40.0, y + h*8.0, 20.0, VIOLET
                );
            }
        
            if is_key_pressed(KeyCode::Escape) {
                break;
            }

            next_frame().await;
        }
    }
}