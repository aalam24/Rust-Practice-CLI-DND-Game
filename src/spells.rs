use crate::classes::Class;

pub struct SpellSlots {
    pub cantrips : i32,
    pub level_1: (i32, i32), // (current, max)
    pub level_2: (i32, i32),
    pub level_3: (i32, i32),
    pub level_4: (i32, i32),
    pub level_5: (i32, i32),
    pub level_6: (i32, i32),
    pub level_7: (i32, i32),
    pub level_8: (i32, i32),
    pub level_9: (i32, i32),
}

impl SpellSlots {
    fn new_slots(cantrips: i32, l1 : i32, l2: i32, l3: i32, l4: i32, l5: i32, l6: i32, l7: i32, l8: i32, l9: i32,) -> Self {
        SpellSlots {
            cantrips,
            level_1: (l1, l1),
            level_2: (l2, l2),
            level_3: (l3, l3),
            level_4: (l4, l4),
            level_5: (l5, l5),
            level_6: (l6, l6),
            level_7: (l7, l7),
            level_8: (l8, l8),
            level_9: (l9, l9),
        }
    }
    pub fn full_caster(level: i32) -> Self {
        match level {
            1 => Self::new_slots(3, 2, 0, 0, 0, 0, 0, 0, 0, 0),
            2 => Self::new_slots(3, 3, 0, 0, 0, 0, 0, 0, 0, 0),
            3 => Self::new_slots(3, 4, 2, 0, 0, 0, 0, 0, 0, 0),
            4 => Self::new_slots(4, 4, 3, 0, 0, 0, 0, 0, 0, 0),
            5 => Self::new_slots(4, 4, 3, 2, 0, 0, 0, 0, 0, 0),
            6 => Self::new_slots(4, 4, 3, 3, 0, 0, 0, 0, 0, 0),
            7 => Self::new_slots(4, 4, 3, 3, 1, 0, 0, 0, 0, 0),
            8 => Self::new_slots(4, 4, 3, 3, 2, 0, 0, 0, 0, 0),
            9 => Self::new_slots(4, 4, 3, 3, 3, 1, 0, 0, 0, 0),
            10 => Self::new_slots(5, 4, 3, 3, 3, 2, 0, 0, 0, 0),
            11 => Self::new_slots(5, 4, 3, 3, 3, 2, 1, 0, 0, 0),
            12 => Self::new_slots(5, 4, 3, 3, 3, 2, 1, 0, 0, 0),
            13 => Self::new_slots(5, 4, 3, 3, 3, 2, 1, 1, 0, 0),
            14 => Self::new_slots(5, 4, 3, 3, 3, 2, 1, 1, 0, 0),
            15 => Self::new_slots(5, 4, 3, 3, 3, 2, 1, 1, 1, 0),
            16 => Self::new_slots(5, 4, 3, 3, 3, 2, 1, 1, 1, 0),
            17 => Self::new_slots(5, 4, 3, 3, 3, 2, 1, 1, 1, 1),
            18 => Self::new_slots(5, 4, 3, 3, 3, 3, 1, 1, 1, 1),
            19 => Self::new_slots(5, 4, 3, 3, 3, 3, 2, 1, 1, 1),
            20 => Self::new_slots(5, 4, 3, 3, 3, 3, 2, 2, 1, 1),
            _ => Self::new_slots(0, 0, 0, 0, 0, 0, 0, 0, 0, 0),
        }
    }
    pub fn use_slot(&mut self, spell_level: u32) -> Result<(), String> {
        let slot = match spell_level {
            1 => &mut self.level_1,
            2 => &mut self.level_2,
            3 => &mut self.level_3,
            4 => &mut self.level_4,
            5 => &mut self.level_5,
            6 => &mut self.level_6,
            7 => &mut self.level_7,
            8 => &mut self.level_8,
            9 => &mut self.level_9,
            _ => return Err("Invalid spell level".to_string()),
        };
        if slot.0 > 0 {
            slot.0 -= 1;
            Ok(())
        } else {
            Err(format!("No spell slots left for level {}", spell_level))
        }
    }
}