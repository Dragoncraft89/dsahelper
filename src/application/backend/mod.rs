use std::hash::{Hash, Hasher};

#[derive(Clone)]
pub enum Stat {
    Attribute(&'static str, &'static str),
    Ability(&'static str, Vec<&'static str>),
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

pub struct StatDescription {
    pub stat: Stat,
    pub min: i32,
    pub max: i32,
}

pub trait ModifierValue {
    fn name(&self) -> String;
    fn get_modifier(&self, s: &Stat, val: i32) -> i32;
}

pub struct Modifier {
    pub name: &'static str,
    values: fn(&Player) -> Vec<Box<ModifierValue>>,
}

impl Modifier {
    pub fn new(name: &'static str, values: fn(&Player) -> Vec<Box<ModifierValue>>) -> Modifier {
        Modifier {
            name: name,
            values: values,
        }
    }
    pub fn get_values(&self, p: &Player) -> Vec<Box<ModifierValue>> {
        (self.values)(p)
    }
}

pub enum CategoryEntry {
    Stat(StatDescription),
    Modifier(Modifier),
}

pub struct StatCategory {
    pub name: &'static str,
    pub entries: Vec<CategoryEntry>,
}

impl StatCategory {
    pub fn new(name: &'static str) -> StatCategory {
        StatCategory {
            name: name,
            entries: Vec::new(),
        }
    }

    pub fn add_stat(&mut self, stat: Stat, min: i32, max: i32) {
        self.entries.push(CategoryEntry::Stat(StatDescription {
            stat: stat,
            min: min,
            max: max,
        }));
    }

    pub fn add_modifier(&mut self, modifier: Modifier) {
        self.entries.push(CategoryEntry::Modifier(modifier));
    }
}

pub struct CharacterSheet {
    categories: Vec<StatCategory>,

    calc: fn(&CharacterSheet, &Player, &StatCategory, &Stat) -> i32,
}

impl CharacterSheet {
    pub fn new(
        calc: fn(&CharacterSheet, &Player, &StatCategory, &Stat) -> i32,
    ) -> CharacterSheet {
        CharacterSheet {
            categories: Vec::new(),
            calc: calc,
        }
    }

    pub fn add_category(&mut self, s: StatCategory) {
        self.categories.push(s);
    }

    pub fn categories(&self) -> &Vec<StatCategory> {
        &self.categories
    }

    pub fn categories_mut(&mut self) -> &mut Vec<StatCategory> {
        &mut self.categories
    }

    pub fn calc_value(&self, p: &Player, c: &StatCategory, s: &Stat) -> i32 {
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
    fn get_time(&self) -> (i32, i32);
    fn set_time(&mut self, hour: i32, minute: i32);

    fn get_date(&self) -> (i32, i32, i32);
    fn set_date(&mut self, day: i32, month: i32, year: i32);

    fn advance_time(&mut self, t: TimeUnits);

    fn get_month_name(&self, month: i32) -> &'static str;

    fn minutes_per_hour(&mut self) -> i32;
    fn hours_per_day(&mut self) -> i32;
    fn days_per_week(&mut self) -> i32;
    fn days_per_month(&mut self, month: i32) -> i32;
    fn months_per_year(&mut self) -> i32;

    fn morning(&self) -> (i32, i32);
    fn noon(&self) -> (i32, i32);
    fn evening(&self) -> (i32, i32);
}

pub trait Player {
    fn name(&self) -> &String;
    fn set_name(&mut self, name: String);

    fn get_value(&self, s: &Stat) -> i32;
    fn set_value(&mut self, s: Stat, val: i32);

    fn get_modifier(&self, s: &String) -> &ModifierValue;
    fn set_modifier(&mut self, s: String, modifier: Box<ModifierValue>);
}

pub trait PenAndPaperBackend {
    fn calendar(&mut self) -> &mut PenAndPaperCalendar;

    fn add_player(&mut self, name: String) -> &Player;
    fn get_player(&mut self, pos: usize) -> &mut Player;
    fn remove_player(&mut self, pos: usize);

    fn character_sheet(&mut self) -> CharacterSheet;
}
