use std::io::{self, Write};

#[derive(Debug, Clone)]
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

impl AbilityScores {
    pub fn point_buy() -> Self {
        let mut scores = AbilityScores {
            strength: 8,
            dexterity: 8,
            constitution: 8,
            intelligence: 8,
            wisdom: 8,
            charisma: 8,
        };
        println!("\n=== POINT BUY SYSTEM ===");
        println!("You have 27 points to spend.");
        println!("All abilities start at 8. You can increase them up to 15.");
        println!("\nCosts: 8→9 (1pt), 9→10 (1pt), 10→11 (1pt), 11→12 (1pt),");
        println!("       12→13 (1pt), 13→14 (2pts), 14→15 (2pts)");
        println!("\nCommands:");
        println!("  str/dex/con/int/wis/cha <value>  - Set ability to value (8-15)");
        println!("  show                              - Display current scores");
        println!("  reset                             - Reset all to 8");
        println!("  done                              - Finish (must spend all points)");
        loop {
            let points_used = Self::calculate_points_used(&scores);
            let points_left = 27 - points_used;

            println!("\n--- Points: {}/27 remaining ---", points_left);
            Self::display_scores(&scores);

            print!("\n> ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let input = input.trim().to_lowercase();

            if input == "done" {
                if points_left == 0 {
                    println!("\n Ability scores locked in.");
                    break;
                } else if points_left > 0 {
                    println!("You still have {} points left to spend.", points_left);
                    continue;
                } else if points_left < 0 {
                    println!("You're over by {} points! Adjust your scores.", -points_left);
                    continue;
                }
            }
            if input == "show" {
                Self::display_scores(&scores);
                continue;
            }

            if input == "reset" {
                scores = AbilityScores {
                    strength: 8,
                    dexterity: 8,
                    constitution: 8,
                    intelligence: 8,
                    wisdom: 8,
                    charisma: 8
                };
                println!("All ability scores have reset to 8.");
                continue;
            }

            // Parse <ability> <value> settings
            let parts: Vec<&str> = input.split_whitespace().collect();
            // This works by checking the values between the whitespace when the user enters ability + value, if this is not 2 parts, then it notifies the user of invalid format
            if parts.len() != 2 {
                println!("Invalid command format. Use '<ability> <value>'.");
                continue;
            }
            let ability = parts[0];
            let value = match parts[1].parse::<i32>() {
                Ok(v) => v, 
                Err(_) => {
                    println!("Invalid value. Please enter a number between 8 and 15.");
                    continue;}
            };

            if value <8 || value > 15 {
                println!("Value must be between 8 and 15.");
                continue;
            }

            let old_scores = scores.clone();
            match ability {
                "str" | "strength"=> scores.strength = value,
                "dex" | "dexterity" => scores.dexterity = value,
                "con" | "constitution" => scores.constitution = value,
                "int" | "intelligence" => scores.intelligence = value,
                "wis" | "wisdom" => scores.wisdom = value,
                "cha" | "charisma" => scores.charisma = value,
                _ => {
                    println!("Unknown ability '{}'. Use str, dex, con, int, wis, or cha.", ability);
                    continue;
                }
            }
            // Check if points are exceeded after changing
            let new_points_used = Self::calculate_points_used(&scores);
            if new_points_used > 27 {
                scores = old_scores;
                println!("You cannot afford that! You'd need {} points. (Have 27)", new_points_used);
            } else {
                println!("Set {} to {}.", ability.to_uppercase(), value);
            }

        }
        scores
    }

    fn cost_for_score(score: i32) -> i32 {
        // Function to return points needed to reach a given score from 8
        match score {
            8 => 0,
            9 => 1,
            10 => 2,
            11 => 3,
            12 => 4,
            13 => 5,
            14 => 7,
            15 => 9,
            _ => 0,
        }
    }

    fn calculate_points_used(scores: &AbilityScores) -> i32 {
        Self::cost_for_score(scores.strength) +
        Self::cost_for_score(scores.dexterity) +
        Self::cost_for_score(scores.constitution) +
        Self::cost_for_score(scores.intelligence) +
        Self::cost_for_score(scores.wisdom) +
        Self::cost_for_score(scores.charisma)
    }

    fn display_scores(scores: &AbilityScores) {
        println!("STR: {} ({:+})  |  DEX: {} ({:+})  |  CON: {} ({:+})",
            scores.strength, scores.str_mod(),
            scores.dexterity, scores.dex_mod(),
            scores.constitution, scores.con_mod());
        println!("INT: {} ({:+})  |  WIS: {} ({:+})  |  CHA: {} ({:+})",
            scores.intelligence, scores.int_mod(),
            scores.wisdom, scores.wis_mod(),
            scores.charisma, scores.cha_mod());
    }
}
