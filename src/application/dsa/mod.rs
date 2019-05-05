use std::collections::HashMap;
use std::cmp::min;

use crate::application::backend::*;

pub struct AventurienCalendar {
}

pub struct DSAPlayer {
    _name: String,

    character_sheet: HashMap<Stat, i8>
}

impl Player for DSAPlayer {
    fn name(&self) -> &String {
        &self._name
    }

    fn set_name(&mut self, name: String) {
        self._name = name
    }

    fn get_value(&self, s: &Stat) -> i8 {
        if self.character_sheet.contains_key(s) {
            self.character_sheet[s]
        } else {
            0
        }
    }

    fn set_value(&mut self, s: Stat, val: i8) {
        self.character_sheet.insert(s, val);
    }
}

impl PenAndPaperCalendar for AventurienCalendar {
}

pub struct DSABackend {
    cal: AventurienCalendar,
    players: Vec<DSAPlayer>
}

impl DSABackend {
    pub fn new() -> DSABackend {
        DSABackend { cal: AventurienCalendar {}, players: Vec::new() }
    }
}

impl PenAndPaperBackend for DSABackend {
    fn calendar(&mut self) -> &mut PenAndPaperCalendar {
        &mut self.cal
    }

    fn add_player(&mut self, name: String) -> &Player {
        let mut map = HashMap::new();
        map.insert(Stat::Attribute("Mut".to_string(), "MU".to_string()), 10);
        map.insert(Stat::Attribute("Klugheit".to_string(), "KL".to_string()), 10);
        map.insert(Stat::Attribute("Intuition".to_string(), "IN".to_string()), 10);
        map.insert(Stat::Attribute("Charisma".to_string(), "CH".to_string()), 10);
        map.insert(Stat::Attribute("Fingerfertigkeit".to_string(), "FF".to_string()), 10);
        map.insert(Stat::Attribute("Gewandheit".to_string(), "GE".to_string()), 10);
        map.insert(Stat::Attribute("Konsitution".to_string(), "KO".to_string()), 10);
        map.insert(Stat::Attribute("Körperkraft".to_string(), "KK".to_string()), 10);

        self.players.push(DSAPlayer {_name: name, character_sheet: map});
        self.players.last().unwrap()
    }
    
    fn get_player(&mut self, pos: usize) -> &mut Player {
        &mut self.players[pos]
    }
    
    fn remove_player(&mut self, pos: usize) {
        self.players.remove(pos);
    }

    fn character_sheet(&mut self) -> CharacterSheet {
        fn eval(sheet: &CharacterSheet, p: &Player, _c: &StatCategory, s: &Stat, new_val: i8) -> i8 {
            match s {
                Stat::Attribute(_, _short) => {
                    let category = sheet.get_category(&"Attribute".to_string()).unwrap();
                    new_val + min(0, category.stats.iter().filter(|x| {x.stat != *s}).fold(80 - new_val, |x, s| {x - p.get_value(&s.stat)}))
                },
                Stat::Ability(_, _) => {
                    let fold_categories = |x, c: &StatCategory| {
                        x - c.stats.iter().filter(|x| {x.stat != *s}).fold(0, |x, s| {x + p.get_value(&s.stat)})
                    };
                    
                    new_val - min(0, sheet.categories().iter().filter(|c| {c.name != "Attribute"}).fold(60 - new_val, fold_categories))
                }
            }
        }
        
        fn remaining_points_attributes(_sheet: &CharacterSheet, c: &StatCategory, p: &Player) -> i8 {
            c.stats.iter().fold(80, |x, s| {x - p.get_value(&s.stat)})
        }
        
        fn remaining_points_abilities(sheet: &CharacterSheet, _c: &StatCategory, p: &Player) -> i8 {
            let sum_category = |c: &StatCategory| {
                c.stats.iter().fold(0, |x, s| {x + p.get_value(&s.stat)})
            };
            sheet.categories().iter().filter(|c| {c.name != "Attribute"}).map(sum_category).fold(60, |x, n| { x - n})
        }
        
        let mut sheet = CharacterSheet::new(eval);
        
        let mut attributes = StatCategory::new("Attribute".to_string(), remaining_points_attributes);
        attributes.add_stat(Stat::Attribute("Mut".to_string(), "MU".to_string()), 7, 13);
        attributes.add_stat(Stat::Attribute("Klugheit".to_string(), "KL".to_string()), 7, 13);
        attributes.add_stat(Stat::Attribute("Intuition".to_string(), "IN".to_string()), 7, 13);
        attributes.add_stat(Stat::Attribute("Charisma".to_string(), "CH".to_string()), 7, 13);
        attributes.add_stat(Stat::Attribute("Fingerfertigkeit".to_string(), "FF".to_string()), 7, 13);
        attributes.add_stat(Stat::Attribute("Gewandheit".to_string(), "GE".to_string()), 7, 13);
        attributes.add_stat(Stat::Attribute("Konsitution".to_string(), "KO".to_string()), 7, 13);
        attributes.add_stat(Stat::Attribute("Körperkraft".to_string(), "KK".to_string()), 7, 13);
        sheet.add_category(attributes);

        let mut weapons = StatCategory::new("Waffentalente".to_string(), remaining_points_abilities);
        weapons.add_stat(Stat::Ability("Dolche".to_string(), vec![]), -4, 4);
        weapons.add_stat(Stat::Ability("Fechtwaffen".to_string(), vec![]), -4, 4);
        weapons.add_stat(Stat::Ability("Hiebwaffen".to_string(), vec![]), -4, 4);
        weapons.add_stat(Stat::Ability("Säbel".to_string(), vec![]), -4, 4);
        weapons.add_stat(Stat::Ability("Schwerter".to_string(), vec![]), -4, 4);
        weapons.add_stat(Stat::Ability("Speere".to_string(), vec![]), -4, 4);
        weapons.add_stat(Stat::Ability("Stäbe".to_string(), vec![]), -4, 4);
        weapons.add_stat(Stat::Ability("Infanteriewaffen".to_string(), vec![]), -4, 4);
        weapons.add_stat(Stat::Ability("Anderthalbhänder".to_string(), vec![]), -4, 4);
        weapons.add_stat(Stat::Ability("Kettenwaffen".to_string(), vec![]), -4, 4);
        weapons.add_stat(Stat::Ability("Zweihand Hiebwaffen".to_string(), vec![]), -4, 4);
        weapons.add_stat(Stat::Ability("Zweihand Säbel/Schwerter".to_string(), vec![]), -4, 4);
        weapons.add_stat(Stat::Ability("Raufen".to_string(), vec![]), -4, 4);
        weapons.add_stat(Stat::Ability("Ringen".to_string(), vec![]), -4, 4);
        weapons.add_stat(Stat::Ability("Schilde".to_string(), vec![]), -4, 4);
        sheet.add_category(weapons);
        
        let mut physical = StatCategory::new("Körpertalente".to_string(), remaining_points_abilities);
        physical.add_stat(Stat::Ability("Schleichen".to_string(), vec!["MU".to_string(), "IN".to_string(), "GE".to_string()]), -4, 4);
        physical.add_stat(Stat::Ability("Selbstbeherrschung".to_string(), vec!["MU".to_string(), "KO".to_string(), "KK".to_string()]), -4, 4);
        physical.add_stat(Stat::Ability("Sinnesschärfe".to_string(), vec!["KL".to_string(), "IN".to_string(), "GE".to_string()]), -4, 4);
        physical.add_stat(Stat::Ability("Taschendiebstahl".to_string(), vec!["KL".to_string(), "IN".to_string(), "FF".to_string()]), -4, 4);
        physical.add_stat(Stat::Ability("Klettern".to_string(), vec!["GE".to_string(), "GE".to_string(), "KK".to_string()]), -4, 4);
        physical.add_stat(Stat::Ability("Athletik".to_string(), vec!["GE".to_string(), "GE".to_string(), "IN".to_string()]), -4, 4);
        physical.add_stat(Stat::Ability("Springen".to_string(), vec!["GE".to_string(), "GE".to_string(), "KK".to_string()]), -4, 4);
        physical.add_stat(Stat::Ability("(Ent-)Fesseln".to_string(), vec!["GE".to_string(), "KK".to_string(), "FF".to_string()]), -4, 4);
        sheet.add_category(physical);
        
        sheet
    }
}
