use std::cmp::{max, min};
use std::collections::HashMap;

use crate::application::backend::*;

pub struct AventurienCalendar {
    hour: i8,
    minute: i8,

    day: i8,
    month: i8,
    year: i32,
}

enum CultureMensch {
    Andergaster,
    Aranier,
    Bornlaender,
    Fjarninger,
    Horasier,
    Maraskaner,
    Mhanadistani,
    Mittelreicher,
    Mohas,
    Nivesen,
    Norbarden,
    Nordaventurier,
    Nostrier,
    Novadis,
    Suedaventurier,
    Svelltaler,
    Thorwaller,
    Zyklopaeer,
}

impl ModifierValue for CultureMensch {
    fn name(&self) -> String {
        match self {
            CultureMensch::Andergaster => "Andergaster",
            CultureMensch::Aranier => "Aranier",
            CultureMensch::Bornlaender => "Bornländer",
            CultureMensch::Fjarninger => "Fjarninger",
            CultureMensch::Horasier => "Horasier",
            CultureMensch::Maraskaner => "Maraskaner",
            CultureMensch::Mhanadistani => "Mhanadistani",
            CultureMensch::Mittelreicher => "Mittelreicher",
            CultureMensch::Mohas => "Moha",
            CultureMensch::Nivesen => "Nivese",
            CultureMensch::Norbarden => "Norbarde",
            CultureMensch::Nordaventurier => "Nordaventurier",
            CultureMensch::Nostrier => "Nostrier",
            CultureMensch::Novadis => "Novadis",
            CultureMensch::Suedaventurier => "Suedaventurier",
            CultureMensch::Svelltaler => "Svelltaler",
            CultureMensch::Thorwaller => "Thorwaller",
            CultureMensch::Zyklopaeer => "Zyklopäer",
        }
        .to_string()
    }

    fn get_modifier(&self, s: &Stat, _: i8) -> i8 {
        0
    }
}

enum CultureElf {
    Auelfen,
    Firnelfen,
    Waldelfen,
}

impl ModifierValue for CultureElf {
    fn name(&self) -> String {
        match self {
            CultureElf::Auelfen => "Auelf",
            CultureElf::Firnelfen => "Firnelf",
            CultureElf::Waldelfen => "Waldelf",
        }
        .to_string()
    }

    fn get_modifier(&self, s: &Stat, _: i8) -> i8 {
        match self {
            CultureElf::Auelfen => match s {
                Stat::Ability("Betören", _) => 1,
                Stat::Ability("Boote & Schiffe", _) => 1,
                Stat::Ability("Fährtensuche", _) => 1,
                Stat::Ability("Fischen & Angeln", _) => 2,
                Stat::Ability("Körperbeherrschung", _) => 2,
                Stat::Ability("Musizieren", _) => 2,
                Stat::Ability("Orientierung", _) => 1,
                Stat::Ability("Pflanzenkunde", _) => 1,
                Stat::Ability("Schwimmen", _) => 2,
                Stat::Ability("Singen", _) => 2,
                Stat::Ability("Sinnesschärfe", _) => 1,
                Stat::Ability("Tierkunde", _) => 1,
                Stat::Ability("Verbergen", _) => 1,
                Stat::Ability("Wildnisleben", _) => 1,
                Stat::Attribute(_, "AP") => -43,
                _ => 0,
            },
            CultureElf::Firnelfen => match s {
                Stat::Ability("Fährtensuchen", _) => 2,
                Stat::Ability("Fischen & Angeln", _) => 1,
                Stat::Ability("Klettern", _) => 1,
                Stat::Ability("Körperbeherrschung", _) => 2,
                Stat::Ability("Musizieren", _) => 2,
                Stat::Ability("Orientierung", _) => 2,
                Stat::Ability("Selbstbeherrschung", _) => 1,
                Stat::Ability("Singen", _) => 2,
                Stat::Ability("Sinnesschärfe", _) => 2,
                Stat::Ability("Tierkunde", _) => 2,
                Stat::Ability("Verbergen", _) => 2,
                Stat::Ability("Wildnisleben", _) => 2,
                Stat::Attribute(_, "AP") => -55,
                _ => 0,
            },
            CultureElf::Waldelfen => match s {
                Stat::Ability("Fährtensuchen", _) => 2,
                Stat::Ability("Klettern", _) => 1,
                Stat::Ability("Körperbeherrschung", _) => 2,
                Stat::Ability("Musizieren", _) => 2,
                Stat::Ability("Orientierung", _) => 1,
                Stat::Ability("Pflanzenkunde", _) => 2,
                Stat::Ability("Singen", _) => 2,
                Stat::Ability("Sinnesschärfe", _) => 1,
                Stat::Ability("Tierkunde", _) => 2,
                Stat::Ability("Verbergen", _) => 2,
                Stat::Ability("Wildnisleben", _) => 1,
                Stat::Attribute(_, "AP") => -47,
                _ => 0,
            },
        }
    }
}

enum CultureZwerg {
    Ambosszwerge,
    Brillantzwerge,
    Erzzwerge,
    Huegelzwerge,
}

impl ModifierValue for CultureZwerg {
    fn name(&self) -> String {
        match self {
            CultureZwerg::Ambosszwerge => "Ambosszwerg",
            CultureZwerg::Brillantzwerge => "Brillantzwerg",
            CultureZwerg::Erzzwerge => "Erzzwerg",
            CultureZwerg::Huegelzwerge => "Hügelzwerg",
        }
        .to_string()
    }

    fn get_modifier(&self, s: &Stat, _: i8) -> i8 {
        match self {
            CultureZwerg::Ambosszwerge => match s {
                Stat::Ability("Einschüchtern", _) => 1,
                Stat::Ability("Geschichtswissen", _) => 1,
                Stat::Ability("Kraftakt", _) => 2,
                Stat::Ability("Kriegskunst", _) => 2,
                Stat::Ability("Mechanik", _) => 1,
                Stat::Ability("Metallbearbeitung", _) => 2,
                Stat::Ability("Orientierung", _) => 1,
                Stat::Ability("Sagen & Legenden", _) => 1,
                Stat::Ability("Steinbearbeitung", _) => 2,
                Stat::Ability("Verbergen", _) => 1,
                Stat::Ability("Zechen", _) => 2,
                Stat::Attribute(_, "AP") => -31,
                _ => 0,
            },
            CultureZwerg::Brillantzwerge => match s {
                Stat::Ability("Betören", _) => 1,
                Stat::Ability("Geographie", _) => 2,
                Stat::Ability("Geschichtswissen", _) => 1,
                Stat::Ability("Metallbearbeitung", _) => 1,
                Stat::Ability("Musizieren", _) => 1,
                Stat::Ability("Sagen & Legenden", _) => 1,
                Stat::Ability("Schlösserknacken", _) => 2,
                Stat::Ability("Steinbearbeitung", _) => 1,
                Stat::Ability("Tanzen", _) => 1,
                Stat::Ability("Verbergen", _) => 2,
                Stat::Ability("Zechen", _) => 1,
                Stat::Attribute(_, "AP") => -29,
                _ => 0,
            },
            CultureZwerg::Erzzwerge => match s {
                Stat::Ability("Geschichtswissen", _) => 2,
                Stat::Ability("Götter & Kulte", _) => 2,
                Stat::Ability("Mechanik", _) => 2,
                Stat::Ability("Metallbearbeitung", _) => 1,
                Stat::Ability("Rechnen", _) => 2,
                Stat::Ability("Rechtskunde", _) => 1,
                Stat::Ability("Sagen & Legenden", _) => 2,
                Stat::Ability("Selbstbeherrschung", _) => 1,
                Stat::Ability("Steinbearbeitung", _) => 1,
                Stat::Ability("Verbergen", _) => 2,
                Stat::Ability("Zechen", _) => 1,
                Stat::Attribute(_, "AP") => -34,
                _ => 0,
            },
            CultureZwerg::Huegelzwerge => match s {
                Stat::Ability("Fahrzeuge", _) => 1,
                Stat::Ability("Fischen & Angeln", _) => 1,
                Stat::Ability("Lebensmittelbearbeitung", _) => 2,
                Stat::Ability("Singen", _) => 1,
                Stat::Ability("Tanzen", _) => 1,
                Stat::Ability("Verbergen", _) => 2,
                Stat::Ability("Zechen", _) => 1,
                Stat::Attribute(_, "AP") => -13,
                _ => 0,
            },
        }
    }
}

enum Race {
    Mensch,
    Elf,
    Halbelf,
    Zwerg,
}

impl ModifierValue for Race {
    fn name(&self) -> String {
        match self {
            Race::Mensch => "Mensch".to_string(),
            Race::Elf => "Elf".to_string(),
            Race::Halbelf => "Halbelf".to_string(),
            Race::Zwerg => "Zwerg".to_string(),
        }
    }

    fn get_modifier(&self, s: &Stat, _: i8) -> i8 {
        match self {
            Race::Mensch => match s {
                Stat::Attribute(_, "AP") => 0,
                Stat::Attribute(_, "LE") => 5,
                Stat::Attribute(_, "SE") => -5,
                Stat::Attribute(_, "ZK") => -5,
                Stat::Attribute(_, "G") => 8,
                _ => 0,
            },
            Race::Elf => match s {
                Stat::Attribute(_, "AP") => -18,
                Stat::Attribute(_, "LE") => 2,
                Stat::Attribute(_, "SE") => -4,
                Stat::Attribute(_, "ZK") => -6,
                Stat::Attribute(_, "G") => 8,
                Stat::Attribute(_, "IN") => 1,
                Stat::Attribute(_, "GE") => 1,
                _ => 0,
            },
            Race::Halbelf => match s {
                Stat::Attribute(_, "AP") => 0,
                Stat::Attribute(_, "LE") => 5,
                Stat::Attribute(_, "SK") => -4,
                Stat::Attribute(_, "ZK") => -6,
                Stat::Attribute(_, "G") => 8,
                _ => 0,
            },
            Race::Zwerg => match s {
                Stat::Attribute(_, "AP") => -61,
                Stat::Attribute(_, "LE") => 8,
                Stat::Attribute(_, "SK") => -4,
                Stat::Attribute(_, "ZK") => -4,
                Stat::Attribute(_, "G") => 6,
                Stat::Attribute(_, "KO") => 1,
                Stat::Attribute(_, "KK") => 1,
                _ => 0,
            },
        }
    }
}

enum AttributeBonus {
    MU(i8),
    KL(i8),
    IN(i8),
    CH(i8),
    FF(i8),
    GE(i8),
    KO(i8),
    KK(i8),
}

impl ModifierValue for AttributeBonus {
    fn name(&self) -> String {
        match self {
            AttributeBonus::MU(x) => format!("MU {}", x),
            AttributeBonus::KL(x) => format!("KL {}", x),
            AttributeBonus::IN(x) => format!("IN {}", x),
            AttributeBonus::CH(x) => format!("CH {}", x),
            AttributeBonus::FF(x) => format!("FF {}", x),
            AttributeBonus::GE(x) => format!("GE {}", x),
            AttributeBonus::KO(x) => format!("KO {}", x),
            AttributeBonus::KK(x) => format!("KK {}", x),
        }
    }

    fn get_modifier(&self, s: &Stat, _: i8) -> i8 {
        match (self, s) {
            (AttributeBonus::MU(x), Stat::Attribute(_, "MU")) => *x,
            (AttributeBonus::KL(x), Stat::Attribute(_, "KL")) => *x,
            (AttributeBonus::IN(x), Stat::Attribute(_, "IN")) => *x,
            (AttributeBonus::CH(x), Stat::Attribute(_, "CH")) => *x,
            (AttributeBonus::FF(x), Stat::Attribute(_, "FF")) => *x,
            (AttributeBonus::GE(x), Stat::Attribute(_, "GE")) => *x,
            (AttributeBonus::KO(x), Stat::Attribute(_, "KO")) => *x,
            (AttributeBonus::KK(x), Stat::Attribute(_, "KK")) => *x,
            _ => 0
        }
    }
}

pub struct DSAPlayer {
    _name: String,

    character_sheet: HashMap<Stat, i8>,
    race: Box<ModifierValue>,
    culture: Box<ModifierValue>,
    bonus: Box<ModifierValue>
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

    fn get_modifier(&self, s: &String) -> &ModifierValue {
        match s.as_str() {
            "Rasse" => self.race.as_ref(),
            "Kultur" => self.culture.as_ref(),
            "Eigenschaftsbonus" => self.bonus.as_ref(),
            _ => panic!("Invalid modifier category in DSA backend"),
        }
    }

    fn set_modifier(&mut self, s: String, modifier: Box<ModifierValue>) {
        match s.as_str() {
            "Rasse" => self.race = modifier,
            "Kultur" => self.culture = modifier,
            "Eigenschaftsbonus" => self.bonus = modifier,
            _ => panic!("Invalid modifier category in DSA backend"),
        }
    }
}

impl PenAndPaperCalendar for AventurienCalendar {
    fn get_time(&self) -> (i8, i8) {
        (self.hour, self.minute)
    }

    fn set_time(&mut self, hour: i8, minute: i8) {
        self.hour = min(max(hour, 0), 23);
        self.minute = min(max(minute, 0), 59);
    }

    fn get_date(&self) -> (i8, i8, i32) {
        (self.day, self.month, self.year)
    }

    fn set_date(&mut self, day: i8, month: i8, year: i32) {
        self.day = min(max(day, 1), 30);
        self.month = min(max(month, 1), 12);
        self.year = year;
    }

    fn advance_time(&mut self, t: TimeUnits) {
        match t {
            TimeUnits::Minutes(m) => {
                let minutes = self.minute as i32 + m;
                let mut hours = minutes / 60;
                self.minute = (minutes % 60) as i8;
                if self.minute < 0 {
                    hours -= 1;
                    self.minute += 60;
                }
                self.advance_time(TimeUnits::Hours(hours));
            }
            TimeUnits::Hours(h) => {
                let hours = self.hour as i32 + h;
                let mut days = hours / 24;
                self.hour = (hours % 24) as i8;
                if self.hour < 0 {
                    days -= 1;
                    self.hour += 24;
                }
                self.advance_time(TimeUnits::Days(days));
            }
            TimeUnits::Days(d) => {
                let days = self.day as i32 + d;
                let mut months = (days - 1) / 30;
                self.day = ((days - 1) % 30 + 1) as i8;

                if self.day < 1 {
                    months -= 1;
                    self.day += 30;
                }
                self.advance_time(TimeUnits::Months(months));
            }
            TimeUnits::Weeks(w) => {
                self.advance_time(TimeUnits::Days(7 * w));
            }
            TimeUnits::Months(m) => {
                let months = self.month as i32 + m;
                let mut years = (months - 1) / 12;
                self.month = ((months - 1) % 12 + 1) as i8;

                if self.month < 1 {
                    years -= 1;
                    self.month += 12;
                }
                self.advance_time(TimeUnits::Years(years));
            }
            TimeUnits::Years(y) => {
                self.year += y;
            }
        }
    }

    fn get_month_name(&self, month: i8) -> &'static str {
        match month {
            1 => "Praios",
            2 => "Rondra",
            3 => "Efferd",
            4 => "Travia",
            5 => "Boron",
            6 => "Hesinde",
            7 => "Firun",
            8 => "Tsa",
            9 => "Phex",
            10 => "Peraine",
            11 => "Ingerimm",
            12 => "Rahja",
            _ => "Undefined",
        }
    }

    fn minutes_per_hour(&mut self) -> i8 {
        60
    }
    fn hours_per_day(&mut self) -> i8 {
        24
    }
    fn days_per_week(&mut self) -> i8 {
        7
    }
    fn days_per_month(&mut self, _: i8) -> i8 {
        30
    }
    fn months_per_year(&mut self) -> i8 {
        12
    }

    fn morning(&self) -> (i8, i8) {
        (08, 00)
    }
    fn noon(&self) -> (i8, i8) {
        (12, 00)
    }
    fn evening(&self) -> (i8, i8) {
        (18, 00)
    }
}

pub struct DSABackend {
    cal: AventurienCalendar,
    players: Vec<DSAPlayer>,
}

impl DSABackend {
    pub fn new() -> DSABackend {
        DSABackend {
            cal: AventurienCalendar {
                day: 1,
                month: 1,
                year: 1000,
                hour: 8,
                minute: 0,
            },
            players: Vec::new(),
        }
    }
}

impl PenAndPaperBackend for DSABackend {
    fn calendar(&mut self) -> &mut PenAndPaperCalendar {
        &mut self.cal
    }

    fn add_player(&mut self, name: String) -> &Player {
        let mut map = HashMap::new();
        map.insert(Stat::Attribute("Mut", "MU"), 8);
        map.insert(Stat::Attribute("Klugheit", "KL"), 8);
        map.insert(Stat::Attribute("Intuition", "IN"), 8);
        map.insert(Stat::Attribute("Charisma", "CH"), 8);
        map.insert(Stat::Attribute("Fingerfertigkeit", "FF"), 8);
        map.insert(Stat::Attribute("Gewandheit", "GE"), 8);
        map.insert(Stat::Attribute("Konsitution", "KO"), 8);
        map.insert(Stat::Attribute("Körperkraft", "KK"), 8);

        self.players.push(DSAPlayer {
            _name: name,
            character_sheet: map,
            race: Box::new(Race::Mensch),
            culture: Box::new(CultureMensch::Andergaster),
            bonus: Box::new(AttributeBonus::MU(1))
        });
        self.players.last().unwrap()
    }

    fn get_player(&mut self, pos: usize) -> &mut Player {
        &mut self.players[pos]
    }

    fn remove_player(&mut self, pos: usize) {
        self.players.remove(pos);
    }

    fn character_sheet(&mut self) -> CharacterSheet {
        fn eval(
            sheet: &CharacterSheet,
            p: &Player,
            _c: &StatCategory,
            s: &Stat,
            new_val: i8,
        ) -> i8 {
            match s {
                Stat::Attribute(_, _) => {
                    let category = sheet.get_category(&"Attribute".to_string()).unwrap();
                    new_val
                        + min(
                            0,
                            category
                                .entries
                                .iter()
                                .filter(|x| {
                                    if let CategoryEntry::Stat(x) = x {
                                        x.stat != *s
                                    } else {
                                        false
                                    }
                                })
                                .fold(80 - new_val, |x, s| {
                                    x - if let CategoryEntry::Stat(s) = s {
                                        p.get_value(&s.stat)
                                    } else {
                                        0
                                    }
                                }),
                        )
                }
                Stat::Ability(_, _) => {
                    let fold_categories = |x, c: &StatCategory| {
                        x - c
                            .entries
                            .iter()
                            .filter(|x| {
                                if let CategoryEntry::Stat(x) = x {
                                    x.stat != *s
                                } else {
                                    false
                                }
                            })
                            .fold(0, |x, s| {
                                x + if let CategoryEntry::Stat(s) = s {
                                    p.get_value(&s.stat)
                                } else {
                                    0
                                }
                            })
                    };

                    new_val
                        - min(
                            0,
                            sheet
                                .categories()
                                .iter()
                                .filter(|c| c.name != "Attribute")
                                .fold(60 - new_val, fold_categories),
                        )
                }
            }
        }

        fn calc(_sheet: &CharacterSheet, p: &Player, _c: &StatCategory, s: &Stat) -> i8 {
            let val = p.get_value(s);
            val + p.get_modifier(&"Rasse".to_string()).get_modifier(s, val)
                + p.get_modifier(&"Kultur".to_string()).get_modifier(s, val)
                + p.get_modifier(&"Eigenschaftsbonus".to_string()).get_modifier(s, val)
        }

        let mut sheet = CharacterSheet::new(eval, calc);

        let mut character = StatCategory::new("Charakter");
        let races = Modifier::new("Rasse", |_| {
            vec![
                Box::new(Race::Mensch),
                Box::new(Race::Halbelf),
                Box::new(Race::Elf),
                Box::new(Race::Zwerg),
            ]
        });
        let cultures = Modifier::new("Kultur", |p| {
            match p.get_modifier(&"Rasse".to_string()).name().as_str() {
                "Mensch" => vec![
                    Box::new(CultureMensch::Andergaster),
                    Box::new(CultureMensch::Aranier),
                    Box::new(CultureMensch::Bornlaender),
                    Box::new(CultureMensch::Fjarninger),
                    Box::new(CultureMensch::Horasier),
                    Box::new(CultureMensch::Maraskaner),
                    Box::new(CultureMensch::Mhanadistani),
                    Box::new(CultureMensch::Mittelreicher),
                    Box::new(CultureMensch::Mohas),
                    Box::new(CultureMensch::Nivesen),
                    Box::new(CultureMensch::Norbarden),
                    Box::new(CultureMensch::Nordaventurier),
                    Box::new(CultureMensch::Nostrier),
                    Box::new(CultureMensch::Novadis),
                    Box::new(CultureMensch::Suedaventurier),
                    Box::new(CultureMensch::Svelltaler),
                    Box::new(CultureMensch::Thorwaller),
                    Box::new(CultureMensch::Zyklopaeer),
                ],
                "Halbelf" => vec![
                    Box::new(CultureMensch::Andergaster),
                    Box::new(CultureMensch::Aranier),
                    Box::new(CultureMensch::Bornlaender),
                    Box::new(CultureMensch::Fjarninger),
                    Box::new(CultureMensch::Horasier),
                    Box::new(CultureMensch::Maraskaner),
                    Box::new(CultureMensch::Mhanadistani),
                    Box::new(CultureMensch::Mittelreicher),
                    Box::new(CultureMensch::Mohas),
                    Box::new(CultureMensch::Nivesen),
                    Box::new(CultureMensch::Norbarden),
                    Box::new(CultureMensch::Nordaventurier),
                    Box::new(CultureMensch::Nostrier),
                    Box::new(CultureMensch::Novadis),
                    Box::new(CultureMensch::Suedaventurier),
                    Box::new(CultureMensch::Svelltaler),
                    Box::new(CultureMensch::Thorwaller),
                    Box::new(CultureMensch::Zyklopaeer),
                    Box::new(CultureElf::Auelfen),
                    Box::new(CultureElf::Firnelfen),
                    Box::new(CultureElf::Waldelfen),
                ],
                "Elf" => vec![
                    Box::new(CultureElf::Auelfen),
                    Box::new(CultureElf::Firnelfen),
                    Box::new(CultureElf::Waldelfen),
                ],
                "Zwerg" => vec![
                    Box::new(CultureZwerg::Ambosszwerge),
                    Box::new(CultureZwerg::Brillantzwerge),
                    Box::new(CultureZwerg::Erzzwerge),
                    Box::new(CultureZwerg::Huegelzwerge),
                ],
                x => panic!("Unexpected race modifier: {}", x),
            }
        });
        let bonus = Modifier::new("Eigenschaftsbonus", |p| {
            match p.get_modifier(&"Rasse".to_string()).name().as_str() {
                "Mensch" => vec![
                    Box::new(AttributeBonus::MU(1)),
                    Box::new(AttributeBonus::KL(1)),
                    Box::new(AttributeBonus::IN(1)),
                    Box::new(AttributeBonus::CH(1)),
                    Box::new(AttributeBonus::FF(1)),
                    Box::new(AttributeBonus::GE(1)),
                    Box::new(AttributeBonus::KO(1)),
                    Box::new(AttributeBonus::KK(1)),
                ],
                "Halbelf" => vec![
                    Box::new(AttributeBonus::MU(1)),
                    Box::new(AttributeBonus::KL(1)),
                    Box::new(AttributeBonus::IN(1)),
                    Box::new(AttributeBonus::CH(1)),
                    Box::new(AttributeBonus::FF(1)),
                    Box::new(AttributeBonus::GE(1)),
                    Box::new(AttributeBonus::KO(1)),
                    Box::new(AttributeBonus::KK(1)),
                ],
                "Elf" => vec![
                    Box::new(AttributeBonus::KL(-2)),
                    Box::new(AttributeBonus::KK(-2)),
                ],
                "Zwerg" => vec![
                    Box::new(AttributeBonus::CH(-2)),
                    Box::new(AttributeBonus::GE(-2)),
                ],
                x => panic!("Unexpected race modifier: {}", x),
            }
        });

        character.add_modifier(races);
        character.add_modifier(cultures);
        character.add_modifier(bonus);
        sheet.add_category(character);

        let mut attributes = StatCategory::new("Attribute");
        attributes.add_stat(Stat::Attribute("Mut", "MU"), 7, 13);
        attributes.add_stat(Stat::Attribute("Klugheit", "KL"), 7, 13);
        attributes.add_stat(Stat::Attribute("Intuition", "IN"), 7, 13);
        attributes.add_stat(Stat::Attribute("Charisma", "CH"), 7, 13);
        attributes.add_stat(Stat::Attribute("Fingerfertigkeit", "FF"), 7, 13);
        attributes.add_stat(Stat::Attribute("Gewandheit", "GE"), 7, 13);
        attributes.add_stat(Stat::Attribute("Konstitution", "KO"), 7, 13);
        attributes.add_stat(Stat::Attribute("Körperkraft", "KK"), 7, 13);
        sheet.add_category(attributes);

        let mut weapons = StatCategory::new("Waffentalente");
        weapons.add_stat(Stat::Ability("Dolche", vec![]), -4, 4);
        weapons.add_stat(Stat::Ability("Fechtwaffen", vec![]), -4, 4);
        weapons.add_stat(Stat::Ability("Hiebwaffen", vec![]), -4, 4);
        weapons.add_stat(Stat::Ability("Säbel", vec![]), -4, 4);
        weapons.add_stat(Stat::Ability("Schwerter", vec![]), -4, 4);
        weapons.add_stat(Stat::Ability("Speere", vec![]), -4, 4);
        weapons.add_stat(Stat::Ability("Stäbe", vec![]), -4, 4);
        weapons.add_stat(Stat::Ability("Infanteriewaffen", vec![]), -4, 4);
        weapons.add_stat(Stat::Ability("Anderthalbhänder", vec![]), -4, 4);
        weapons.add_stat(Stat::Ability("Kettenwaffen", vec![]), -4, 4);
        weapons.add_stat(Stat::Ability("Zweihand Hiebwaffen", vec![]), -4, 4);
        weapons.add_stat(Stat::Ability("Zweihand Säbel/Schwerter", vec![]), -4, 4);
        weapons.add_stat(Stat::Ability("Raufen", vec![]), -4, 4);
        weapons.add_stat(Stat::Ability("Ringen", vec![]), -4, 4);
        weapons.add_stat(Stat::Ability("Schilde", vec![]), -4, 4);
        sheet.add_category(weapons);

        let mut physical = StatCategory::new("Körpertalente");
        physical.add_stat(Stat::Ability("Schleichen", vec!["MU", "IN", "GE"]), -4, 4);
        physical.add_stat(
            Stat::Ability("Selbstbeherrschung", vec!["MU", "KO", "KK"]),
            -4,
            4,
        );
        physical.add_stat(
            Stat::Ability("Sinnesschärfe", vec!["KL", "IN", "GE"]),
            -4,
            4,
        );
        physical.add_stat(
            Stat::Ability("Taschendiebstahl", vec!["KL", "IN", "FF"]),
            -4,
            4,
        );
        physical.add_stat(Stat::Ability("Klettern", vec!["GE", "GE", "KK"]), -4, 4);
        physical.add_stat(Stat::Ability("Athletik", vec!["GE", "GE", "IN"]), -4, 4);
        physical.add_stat(Stat::Ability("Springen", vec!["GE", "GE", "KK"]), -4, 4);
        physical.add_stat(
            Stat::Ability("(Ent-)Fesseln", vec!["GE", "KK", "FF"]),
            -4,
            4,
        );
        sheet.add_category(physical);

        sheet
    }
}
