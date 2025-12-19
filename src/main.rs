mod player;
mod classes;
mod races;
mod abilities;
mod spells;

use crate::player::Player;
use crate::races::Race;
use crate::classes::Class;
use crate::abilities::AbilityScores;
use crate::spells::SpellSlots;
use ::rand::Rng;
use macroquad::{prelude::*, ui::{hash, root_ui, widgets}};

enum GameState {
    MainMenu,
    CharacterCreation,
    Exploring,
    InCombat,
    Paused,
    GameOver
}
fn roll_dice(numb_dice: u32, sides: u32, modifier: i32) -> i32 {
    let mut total = 0;
    for _ in 0..numb_dice {
        let roll = ::rand::random::<u32>() % sides + 1;
        total += roll as i32;
    }
    return total + modifier;
}


pub fn window_conf() -> macroquad::prelude::Conf {
    Conf {
        window_title: "Character Sheet".to_string(),
        window_width: 1920,
        window_height: 1080,
        window_resizable: true,
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    let mut state = GameState::MainMenu;
    let mut player_name = String::new();

    // Main menu game loop, runs forever
    loop {
        clear_background(DARKGRAY);
        match state {
            GameState::MainMenu => {
                let center_x = screen_width() / 2.0;
                let center_y = screen_height() / 2.0;
                draw_text("DnDueller", center_x - 100.0, center_y - 150.0, 50.0, WHITE);
                draw_text("by: Adnan Alam", center_x - 80.0, center_y - 50.0, 24.0, LIGHTGRAY);

                if root_ui().button(vec2(center_x - 100.0, center_y), "New Character") {
                    state = GameState::CharacterCreation;
                }
                
            }
            GameState::CharacterCreation => {
                let center_x = screen_width() / 2.0;
                let top_y = screen_height() / 10.0;
                // Title
                let title = "Character Creation";
                let title_size = measure_text(title, None, 50, 1.0);
                draw_text(title, center_x - title_size.width / 2.0, top_y, 50.0, WHITE);
                // cool ass who are you? question
                let name_prompt = "Who are you?";
                let name_prompt_size = measure_text(name_prompt, None, 36, 1.0);
                draw_text(name_prompt, center_x - name_prompt_size.width / 2.0, top_y + 30.0, 36.0, WHITE);
                // group to make Input text box from widgets in macroquad
                widgets::Group::new(hash!(), vec2(200.0, 50.0))
                    .position(vec2(center_x - 100.0, top_y + 50.0))
                    .ui(&mut root_ui(), |ui| {
                        widgets::InputText::new(hash!())
                            .size(vec2(200.0, 30.0))
                            .ui(ui, &mut player_name);
                    });
                    player_name = player_name
                    .chars()
                    .filter(|c| c.is_alphabetic() || *c == ' ' || *c == '-' || *c == '\'')
                    .take(24)
                    .collect();

            }
            _ => {
                break;
            }
        }
        next_frame().await;
    }
}