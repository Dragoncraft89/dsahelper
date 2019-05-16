use std::hash::{Hash, Hasher};

#[derive(Clone)]
pub enum Stat {
    Attribute(String, String),
    Ability(String, Vec<String>),
}

impl Eq for Stat {}

impl PartialEq for Stat {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&Stat::Attribute(_, ref a), &Stat::Attribute(_, ref b)) => a == b,
            (&Stat::Ability(ref a, _), &Stat::Ability(ref b, _)) => a == b,
            _ => false,
        }
    }
}

impl Hash for Stat {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            &Stat::Attribute(_, ref a) => a.hash(state),
            &Stat::Ability(ref a, _) => a.hash(state),
        }
    }
}

pub struct RangedStat {
    pub stat: Stat,
    pub min: i8,
    pub max: i8,
}

pub struct StatCategory {
    pub name: String,
    pub stats: Vec<RangedStat>,

    eval: fn(&CharacterSheet, &StatCategory, &Player) -> i8,
}

impl StatCategory {
    pub fn new(
        name: String,
        eval: fn(&CharacterSheet, &StatCategory, &Player) -> i8,
    ) -> StatCategory {
        StatCategory {
            name: name,
            stats: Vec::new(),
            eval: eval,
        }
    }

    pub fn add_stat(&mut self, stat: Stat, min: i8, max: i8) {
        self.stats.push(RangedStat {
            stat: stat,
            min: min,
            max: max,
        });
    }

    pub fn get_remaining_points(&self, sheet: &CharacterSheet, p: &Player) -> i8 {
        (self.eval)(sheet, self, p)
    }
}

pub struct CharacterSheet {
    categories: Vec<StatCategory>,

    evaluator: fn(&CharacterSheet, &Player, &StatCategory, &Stat, i8) -> i8,
    calc: fn(&CharacterSheet, &Player, &StatCategory, &Stat) -> i8,
}

impl CharacterSheet {
    pub fn new(
        evaluator: fn(&CharacterSheet, &Player, &StatCategory, &Stat, i8) -> i8,
        calc: fn(&CharacterSheet, &Player, &StatCategory, &Stat) -> i8,
    ) -> CharacterSheet {
        CharacterSheet {
            categories: Vec::new(),
            evaluator: evaluator,
            calc: calc,
        }
    }

    pub fn get_category(&self, name: &String) -> Option<&StatCategory> {
        for category in self.categories.iter() {
            if category.name == *name {
                return Some(category);
            }
        }

        None
    }

    pub fn add_category(&mut self, s: StatCategory) {
        self.categories.push(s);
    }

    pub fn categories(&self) -> &Vec<StatCategory> {
        &self.categories
    }

    pub fn validate_value(&self, p: &Player, c: &StatCategory, s: &Stat, new_value: i8) -> i8 {
        (self.evaluator)(&self, p, c, s, new_value)
    }

    pub fn calc_value(&self, p: &Player, c: &StatCategory, s: &Stat) -> i8 {
        (self.calc)(&self, p, c, s)
    }
}

#[derive(Copy, Clone)]
pub enum TimeUnits {
    Minutes(i32),
    Hours(i32),
    Days(i32),
    Weeks(i32),
    Months(i32),
    Years(i32),
}

pub trait PenAndPaperCalendar {
    fn get_time(&self) -> (i8, i8);
    fn set_time(&mut self, hour: i8, minute: i8);

    fn get_date(&self) -> (i8, i8, i32);
    fn set_date(&mut self, day: i8, month: i8, year: i32);

    fn advance_time(&mut self, t: TimeUnits);

    fn get_month_name(&self, month: i8) -> &'static str;

    fn minutes_per_hour(&mut self) -> i8;
    fn hours_per_day(&mut self) -> i8;
    fn days_per_week(&mut self) -> i8;
    fn days_per_month(&mut self, month: i8) -> i8;
    fn months_per_year(&mut self) -> i8;

    fn morning(&self) -> (i8, i8);
    fn noon(&self) -> (i8, i8);
    fn evening(&self) -> (i8, i8);
}

pub trait Player {
    fn name(&self) -> &String;
    fn set_name(&mut self, name: String);

    fn get_value(&self, s: &Stat) -> i8;
    fn set_value(&mut self, s: Stat, val: i8);
}

pub trait PenAndPaperBackend {
    fn calendar(&mut self) -> &mut PenAndPaperCalendar;

    fn add_player(&mut self, name: String) -> &Player;
    fn get_player(&mut self, pos: usize) -> &mut Player;
    fn remove_player(&mut self, pos: usize);

    fn character_sheet(&mut self) -> CharacterSheet;
}
