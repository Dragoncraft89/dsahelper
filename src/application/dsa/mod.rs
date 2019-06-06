use std::cmp::{max, min};
use std::collections::HashMap;

use crate::application::backend::*;

pub struct AventurienCalendar {
    hour: i32,
    minute: i32,

    day: i32,
    month: i32,
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
            CultureMensch::Andergaster => "Andergaster (20 AP)",
            CultureMensch::Aranier => "Aranier (26 AP)",
            CultureMensch::Bornlaender => "Bornländer (18 AP)",
            CultureMensch::Fjarninger => "Fjarninger (33 AP)",
            CultureMensch::Horasier => "Horasier (27 AP)",
            CultureMensch::Maraskaner => "Maraskaner (28 AP)",
            CultureMensch::Mhanadistani => "Mhanadistani (26 AP)",
            CultureMensch::Mittelreicher => "Mittelreicher (12 AP)",
            CultureMensch::Mohas => "Moha (38 AP)",
            CultureMensch::Nivesen => "Nivese (37 AP)",
            CultureMensch::Norbarden => "Norbarde (18 AP)",
            CultureMensch::Nordaventurier => "Nordaventurier (25 AP)",
            CultureMensch::Nostrier => "Nostrier (15 AP)",
            CultureMensch::Novadis => "Novadis (25 AP)",
            CultureMensch::Suedaventurier => "Suedaventurier (26 AP)",
            CultureMensch::Svelltaler => "Svelltaler (21 AP)",
            CultureMensch::Thorwaller => "Thorwaller (24 AP)",
            CultureMensch::Zyklopaeer => "Zyklopäer (16 AP)",
        }
        .to_string()
    }

    fn get_modifier(&self, s: &Stat, _: i32) -> i32 {
        match self {
            CultureMensch::Andergaster => match s {
                Stat::Ability("Holzbearbeitung", _) => 2,
                Stat::Ability("Orientierung", _) => 1,
                Stat::Ability("Pflanzenkunde", _) => 1,
                Stat::Ability("Sagen & Legenden", _) => 1,
                Stat::Ability("Tierkunde", _) => 1,
                Stat::Ability("Wildnisleben", _) => 2,

                Stat::Attribute(_, "AP") => -20,
                _ => 0,
            },
            CultureMensch::Aranier => match s {
                Stat::Ability("Betören", _) => 1,
                Stat::Ability("Brett- & Glückspiel", _) => 1,
                Stat::Ability("Gassenwissen", _) => 2,
                Stat::Ability("Handel", _) => 2,
                Stat::Ability("Menschenkenntnis", _) => 1,
                Stat::Ability("Rechnen", _) => 1,
                Stat::Ability("Sagen & Legenden", _) => 1,
                Stat::Ability("Stoffbearbeitung", _) => 1,
                Stat::Ability("Überreden", _) => 2,
                Stat::Attribute(_, "AP") => -26,
                _ => 0,
            },
            CultureMensch::Bornlaender => match s {
                Stat::Ability("Fährtensuche", _) => 1,
                Stat::Ability("Holzbearbeitung", _) => 2,
                Stat::Ability("Lebensmittelbearbeitung", _) => 1,
                Stat::Ability("Orientierung", _) => 1,
                Stat::Ability("Pflanzenkunde", _) => 1,
                Stat::Ability("Wildnisleben", _) => 1,
                Stat::Ability("Zechen", _) => 2,
                Stat::Attribute(_, "AP") => -18,
                _ => 0,
            },
            CultureMensch::Fjarninger => match s {
                Stat::Ability("Einschüchtern", _) => 2,
                Stat::Ability("Fährtensuche", _) => 1,
                Stat::Ability("Körperbeherrschung", _) => 1,
                Stat::Ability("Kraftakt", _) => 2,
                Stat::Ability("Metallbearbeitung", _) => 1,
                Stat::Ability("Orientierung", _) => 2,
                Stat::Ability("Selbstbeherrschung", _) => 1,
                Stat::Ability("Steinbearbeitung", _) => 1,
                Stat::Ability("Wildnisleben", _) => 2,
                Stat::Attribute(_, "AP") => -33,
                _ => 0,
            },
            CultureMensch::Horasier => match s {
                Stat::Ability("Betören", _) => 1,
                Stat::Ability("Boote & Schiffe", _) => 1,
                Stat::Ability("Etikette", _) => 2,
                Stat::Ability("Gassenwissen", _) => 2,
                Stat::Ability("Geographie", _) => 1,
                Stat::Ability("Geschichtswissen", _) => 1,
                Stat::Ability("Handel", _) => 1,
                Stat::Ability("Mechanik", _) => 1,
                Stat::Ability("Rechnen", _) => 2,
                Stat::Ability("Rechtskunde", _) => 2,
                Stat::Ability("Tanzen", _) => 1,
                Stat::Attribute(_, "AP") => -27,
                _ => 0,
            },
            CultureMensch::Maraskaner => match s {
                Stat::Ability("Götter & Kulte", _) => 1,
                Stat::Ability("Heilkunde Gift", _) => 2,
                Stat::Ability("Orientierung", _) => 1,
                Stat::Ability("Pflanzenkunde", _) => 2,
                Stat::Ability("Tierkunde", _) => 2,
                Stat::Ability("Verbergen", _) => 1,
                Stat::Ability("Wildnisleben", _) => 1,
                Stat::Attribute(_, "AP") => -26,
                _ => 0,
            },
            CultureMensch::Mhanadistani => match s {
                Stat::Ability("Brett- & Glückspiel", _) => 2,
                Stat::Ability("Gassenwissen", _) => 2,
                Stat::Ability("Geschichtswissen", _) => 2,
                Stat::Ability("Götter & Kulte", _) => 2,
                Stat::Ability("Handel", _) => 2,
                Stat::Ability("Magiekunde", _) => 1,
                Stat::Ability("Sagen & Legenden", _) => 2,
                Stat::Ability("Überreden", _) => 1,
                Stat::Attribute(_, "AP") => -28,
                _ => 0,
            },
            CultureMensch::Mittelreicher => match s {
                Stat::Ability("Holzbearbeitung", _) => 1,
                Stat::Ability("Metallbearbeitung", _) => 1,
                Stat::Ability("Pflanzenkunde", _) => 1,
                Stat::Ability("Stoffbearbeitung", _) => 1,
                Stat::Ability("Tierkunde", _) => 1,
                Stat::Attribute(_, "AP") => -12,
                _ => 0,
            },
            CultureMensch::Mohas => match s {
                Stat::Ability("Fährtensuche", _) => 1,
                Stat::Ability("Heilkunde Gift", _) => 1,
                Stat::Ability("Körperbeherrschung", _) => 1,
                Stat::Ability("Orientierung", _) => 1,
                Stat::Ability("Pflanzenkunde", _) => 2,
                Stat::Ability("Sagen & Legenden", _) => 1,
                Stat::Ability("Sinnesschärfe", _) => 1,
                Stat::Ability("Tierkunde", _) => 2,
                Stat::Ability("Verbergen", _) => 1,
                Stat::Ability("Wildnisleben", _) => 2,
                Stat::Attribute(_, "AP") => -38,
                _ => 0,
            },
            CultureMensch::Nivesen => match s {
                Stat::Ability("Fährtensuche", _) => 2,
                Stat::Ability("Fahrzeuge", _) => 1,
                Stat::Ability("Orientierung", _) => 2,
                Stat::Ability("Pflanzenkunde", _) => 1,
                Stat::Ability("Sagen & Legenden", _) => 2,
                Stat::Ability("Sinnesschärfe", _) => 1,
                Stat::Ability("Tierkunde", _) => 2,
                Stat::Ability("Verbergen", _) => 1,
                Stat::Ability("Wildnisleben", _) => 2,
                Stat::Attribute(_, "AP") => -37,
                _ => 0,
            },
            CultureMensch::Norbarden => match s {
                Stat::Ability("Fahrzeuge", _) => 2,
                Stat::Ability("Geographie", _) => 2,
                Stat::Ability("Handel", _) => 2,
                Stat::Ability("Orientierung", _) => 1,
                Stat::Ability("Überreden", _) => 1,
                Stat::Ability("Wildnisleben", _) => 1,
                Stat::Attribute(_, "AP") => -18,
                _ => 0,
            },
            CultureMensch::Nordaventurier => match s {
                Stat::Ability("Fährtensuche", _) => 2,
                Stat::Ability("Handel", _) => 1,
                Stat::Ability("Holzbearbeitung", _) => 1,
                Stat::Ability("Lederbearbeitung", _) => 1,
                Stat::Ability("Orientierung", _) => 1,
                Stat::Ability("Pflanzenkunde", _) => 1,
                Stat::Ability("Selbstbeherrschung", _) => 1,
                Stat::Ability("Tierkunde", _) => 1,
                Stat::Ability("Wildnisleben", _) => 1,
                Stat::Ability("Zechen", _) => 1,
                Stat::Attribute(_, "AP") => -25,
                _ => 0,
            },
            CultureMensch::Nostrier => match s {
                Stat::Ability("Fischen & Angeln", _) => 2,
                Stat::Ability("Orientierung", _) => 1,
                Stat::Ability("Pflanzenkunde", _) => 1,
                Stat::Ability("Sagen & Legenden", _) => 1,
                Stat::Ability("Tierkunde", _) => 1,
                Stat::Ability("Wildnisleben", _) => 1,
                Stat::Attribute(_, "AP") => -15,
                _ => 0,
            },
            CultureMensch::Novadis => match s {
                Stat::Ability("Einschüchtern", _) => 2,
                Stat::Ability("Fährtensuche", _) => 1,
                Stat::Ability("Orientierung", _) => 2,
                Stat::Ability("Rechtskunde", _) => 1,
                Stat::Ability("Reiten", _) => 2,
                Stat::Ability("Tierkunde", _) => 1,
                Stat::Ability("Wildnisleben", _) => 2,
                Stat::Attribute(_, "AP") => -25,
                _ => 0,
            },
            CultureMensch::Suedaventurier => match s {
                Stat::Ability("Betören", _) => 1,
                Stat::Ability("Boote & Schiffe", _) => 1,
                Stat::Ability("Fischen & Angeln", _) => 1,
                Stat::Ability("Gassenwissen", _) => 2,
                Stat::Ability("Heilkunde Gift", _) => 1,
                Stat::Ability("Menschenkenntnis", _) => 2,
                Stat::Ability("Überreden", _) => 1,
                Stat::Ability("Willenskraft", _) => 1,
                Stat::Attribute(_, "AP") => -26,
                _ => 0,
            },
            CultureMensch::Svelltaler => match s {
                Stat::Ability("Fährtensuche", _) => 2,
                Stat::Ability("Fahrzeuge", _) => 2,
                Stat::Ability("Orientierung", _) => 1,
                Stat::Ability("Pflanzenkunde", _) => 1,
                Stat::Ability("Sagen & Legenden", _) => 1,
                Stat::Ability("Tierkunde", _) => 1,
                Stat::Ability("Wildnisleben", _) => 1,
                Stat::Attribute(_, "AP") => -21,
                _ => 0,
            },
            CultureMensch::Thorwaller => match s {
                Stat::Ability("Boote & Schiffe", _) => 1,
                Stat::Ability("Einschüchtern", _) => 1,
                Stat::Ability("Fischen & Angeln", _) => 2,
                Stat::Ability("Geographie", _) => 1,
                Stat::Ability("Holzbearbeitung", _) => 2,
                Stat::Ability("Kraftakt", _) => 2,
                Stat::Ability("Orientierung", _) => 1,
                Stat::Ability("Sagen & Legenden", _) => 1,
                Stat::Ability("Zechen", _) => 2,
                Stat::Attribute(_, "AP") => -24,
                _ => 0,
            },
            CultureMensch::Zyklopaeer => match s {
                Stat::Ability("Boote & Schiffe", _) => 1,
                Stat::Ability("Fischen & Angeln", _) => 2,
                Stat::Ability("Geschichtswissen", _) => 1,
                Stat::Ability("Götter & Kulte", _) => 2,
                Stat::Ability("Musizieren", _) => 1,
                Stat::Ability("Rechnen", _) => 1,
                Stat::Ability("Rechtskunde", _) => 1,
                Stat::Ability("Sagen & Legenden", _) => 1,
                Stat::Ability("Tanzen", _) => 1,
                Stat::Attribute(_, "AP") => -16,
                _ => 0,
            },
        }
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
            CultureElf::Auelfen => "Auelf (43 AP)",
            CultureElf::Firnelfen => "Firnelf (55 AP)",
            CultureElf::Waldelfen => "Waldelf (47 AP)",
        }
        .to_string()
    }

    fn get_modifier(&self, s: &Stat, _: i32) -> i32 {
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
            CultureZwerg::Ambosszwerge => "Ambosszwerg (31 AP)",
            CultureZwerg::Brillantzwerge => "Brillantzwerg (29 AP)",
            CultureZwerg::Erzzwerge => "Erzzwerg (34 AP)",
            CultureZwerg::Huegelzwerge => "Hügelzwerg (13 AP)",
        }
        .to_string()
    }

    fn get_modifier(&self, s: &Stat, _: i32) -> i32 {
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
            Race::Mensch => "Mensch (0 AP)",
            Race::Elf => "Elf (18 AP)",
            Race::Halbelf => "Halbelf (0 AP)",
            Race::Zwerg => "Zwerg (61 AP)",
        }
        .to_string()
    }

    fn get_modifier(&self, s: &Stat, _: i32) -> i32 {
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
    MU(i32),
    KL(i32),
    IN(i32),
    CH(i32),
    FF(i32),
    GE(i32),
    KO(i32),
    KK(i32),
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

    fn get_modifier(&self, s: &Stat, _: i32) -> i32 {
        match (self, s) {
            (AttributeBonus::MU(x), Stat::Attribute(_, "MU")) => *x,
            (AttributeBonus::KL(x), Stat::Attribute(_, "KL")) => *x,
            (AttributeBonus::IN(x), Stat::Attribute(_, "IN")) => *x,
            (AttributeBonus::CH(x), Stat::Attribute(_, "CH")) => *x,
            (AttributeBonus::FF(x), Stat::Attribute(_, "FF")) => *x,
            (AttributeBonus::GE(x), Stat::Attribute(_, "GE")) => *x,
            (AttributeBonus::KO(x), Stat::Attribute(_, "KO")) => *x,
            (AttributeBonus::KK(x), Stat::Attribute(_, "KK")) => *x,
            _ => 0,
        }
    }
}

pub struct DSAPlayer {
    _name: String,

    character_sheet: HashMap<Stat, i32>,
    race: Box<ModifierValue>,
    culture: Box<ModifierValue>,
    bonus: Box<ModifierValue>,
}

impl Player for DSAPlayer {
    fn name(&self) -> &String {
        &self._name
    }

    fn set_name(&mut self, name: String) {
        self._name = name
    }

    fn get_value(&self, s: &Stat) -> i32 {
        if self.character_sheet.contains_key(s) {
            self.character_sheet[s]
        } else {
            0
        }
    }

    fn set_value(&mut self, s: Stat, val: i32) {
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
    fn get_time(&self) -> (i32, i32) {
        (self.hour, self.minute)
    }

    fn set_time(&mut self, hour: i32, minute: i32) {
        self.hour = min(max(hour, 0), 23);
        self.minute = min(max(minute, 0), 59);
    }

    fn get_date(&self) -> (i32, i32, i32) {
        (self.day, self.month, self.year)
    }

    fn set_date(&mut self, day: i32, month: i32, year: i32) {
        self.day = min(max(day, 1), 30);
        self.month = min(max(month, 1), 12);
        self.year = year;
    }

    fn advance_time(&mut self, t: TimeUnits) {
        match t {
            TimeUnits::Minutes(m) => {
                let minutes = self.minute + m;
                let mut hours = minutes / 60;
                self.minute = minutes % 60;
                if self.minute < 0 {
                    hours -= 1;
                    self.minute += 60;
                }
                self.advance_time(TimeUnits::Hours(hours));
            }
            TimeUnits::Hours(h) => {
                let hours = self.hour + h;
                let mut days = hours / 24;
                self.hour = hours % 24;
                if self.hour < 0 {
                    days -= 1;
                    self.hour += 24;
                }
                self.advance_time(TimeUnits::Days(days));
            }
            TimeUnits::Days(d) => {
                let days = self.day + d;
                let mut months = (days - 1) / 30;
                self.day = (days - 1) % 30 + 1;

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
                let months = self.month + m;
                let mut years = (months - 1) / 12;
                self.month = (months - 1) % 12 + 1;

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

    fn get_month_name(&self, month: i32) -> &'static str {
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

    fn minutes_per_hour(&mut self) -> i32 {
        60
    }
    fn hours_per_day(&mut self) -> i32 {
        24
    }
    fn days_per_week(&mut self) -> i32 {
        7
    }
    fn days_per_month(&mut self, _: i32) -> i32 {
        30
    }
    fn months_per_year(&mut self) -> i32 {
        12
    }

    fn morning(&self) -> (i32, i32) {
        (08, 00)
    }
    fn noon(&self) -> (i32, i32) {
        (12, 00)
    }
    fn evening(&self) -> (i32, i32) {
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
            bonus: Box::new(AttributeBonus::MU(1)),
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
        fn calc(sheet: &CharacterSheet, p: &Player, c: &StatCategory, s: &Stat) -> i32 {
            let mut val = p.get_value(s);
            val = val
                + p.get_modifier(&"Rasse".to_string()).get_modifier(s, val)
                + p.get_modifier(&"Kultur".to_string()).get_modifier(s, val)
                + p.get_modifier(&"Eigenschaftsbonus".to_string())
                    .get_modifier(s, val);

            match s {
                Stat::Attribute(_, "LeP") => {
                    val += calc(sheet, p, c, &Stat::Attribute("Konstitution", "KO")) * 2
                }
                Stat::Attribute(_, "SK") => {
                    val += (calc(sheet, p, c, &Stat::Attribute("Mut", "MU"))
                        + calc(sheet, p, c, &Stat::Attribute("Klugheit", "KL"))
                        + calc(sheet, p, c, &Stat::Attribute("Intuition", "IN")))
                        / 6
                }
                Stat::Attribute(_, "ZK") => {
                    val += (calc(sheet, p, c, &Stat::Attribute("Konstitution", "KO")) * 2
                        + calc(sheet, p, c, &Stat::Attribute("Körperkraft", "KK")))
                        / 6
                }
                Stat::Attribute(_, "AW") => {
                    val += calc(sheet, p, c, &Stat::Attribute("Gewandtheit", "GE")) / 2
                }
                Stat::Attribute(_, "INI") => {
                    val += (calc(sheet, p, c, &Stat::Attribute("Mut", "MU"))
                        + calc(sheet, p, c, &Stat::Attribute("Gewandtheit", "GE")))
                        / 2
                }
                Stat::Attribute(_, "Schips") => val += 3,
                Stat::Attribute(_, "AP") => {
                    val -= sheet
                        .get_category("Attribute")
                        .unwrap()
                        .entries
                        .iter()
                        .fold(0, |val, entry| match entry {
                            CategoryEntry::Stat(stat) => {
                                val + match p.get_value(&stat.stat) {
                                    09 => 15,
                                    10 => 2 * 15,
                                    11 => 3 * 15,
                                    12 => 4 * 15,
                                    13 => 5 * 15,
                                    14 => 6 * 15,
                                    15 => 6 * 15 + 30,
                                    16 => 6 * 15 + 45,
                                    17 => 6 * 15 + 60,
                                    18 => 6 * 15 + 75,
                                    19 => 6 * 17 + 90,
                                    _ => 0,
                                }
                            }
                            _ => 0,
                        })
                }
                Stat::Calculated(name) => {
                    let len = name.len();
                    if name.ends_with(" - Attacke") {
                        let ability = c.find_stat(&name.split_at(len - 10).0.to_string()).unwrap();
                        let level = p.get_value(&ability.stat);

                        val += level + (calc(sheet, p, c, &Stat::Attribute("Mut", "MU")) - 8) / 3;
                    }
                    if name.ends_with(" - Parade") {
                        let ability = c.find_stat(&name.split_at(len - 9).0.to_string()).unwrap();
                        let level = p.get_value(&ability.stat);

                        let max = match &ability.stat {
                            Stat::Ability(_, attribs) => attribs
                                .iter()
                                .map(|x| (calc(sheet, p, c, &Stat::Attribute("", x)) - 8) / 3)
                                .max().unwrap(),
                            _ => 0,
                        };

                        val += level + max;
                    }
                    if name.ends_with(" - Fernkampf") {
                        let ability = c.find_stat(&name.split_at(len - 12).0.to_string()).unwrap();
                        let level = p.get_value(&ability.stat);

                        val += level
                            + (calc(sheet, p, c, &Stat::Attribute("Fingerfertigkeit", "FF")) - 8) / 3;
                    }
                }
                _ => (),
            }

            val
        }

        let mut sheet = CharacterSheet::new(calc);

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
                "Mensch (0 AP)" => vec![
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
                "Halbelf (0 AP)" => vec![
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
                "Elf (18 AP)" => vec![
                    Box::new(CultureElf::Auelfen),
                    Box::new(CultureElf::Firnelfen),
                    Box::new(CultureElf::Waldelfen),
                ],
                "Zwerg (61 AP)" => vec![
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
                "Mensch (0 AP)" => vec![
                    Box::new(AttributeBonus::MU(1)),
                    Box::new(AttributeBonus::KL(1)),
                    Box::new(AttributeBonus::IN(1)),
                    Box::new(AttributeBonus::CH(1)),
                    Box::new(AttributeBonus::FF(1)),
                    Box::new(AttributeBonus::GE(1)),
                    Box::new(AttributeBonus::KO(1)),
                    Box::new(AttributeBonus::KK(1)),
                ],
                "Halbelf (0 AP)" => vec![
                    Box::new(AttributeBonus::MU(1)),
                    Box::new(AttributeBonus::KL(1)),
                    Box::new(AttributeBonus::IN(1)),
                    Box::new(AttributeBonus::CH(1)),
                    Box::new(AttributeBonus::FF(1)),
                    Box::new(AttributeBonus::GE(1)),
                    Box::new(AttributeBonus::KO(1)),
                    Box::new(AttributeBonus::KK(1)),
                ],
                "Elf (18 AP)" => vec![
                    Box::new(AttributeBonus::KL(-2)),
                    Box::new(AttributeBonus::KK(-2)),
                ],
                "Zwerg (61 AP)" => vec![
                    Box::new(AttributeBonus::CH(-2)),
                    Box::new(AttributeBonus::GE(-2)),
                ],
                x => panic!("Unexpected race modifier: {}", x),
            }
        });

        character.add_modifier(races);
        character.add_modifier(cultures);
        character.add_modifier(bonus);
        character.add_stat(Stat::Attribute("Abenterpunkte", "AP"), 0, 5000);
        character.add_stat(Stat::Attribute("Lebensenergie", "LeP"), 0, 127);
        character.add_stat(Stat::Attribute("Astralenergie", "AsP"), 0, 127);
        character.add_stat(Stat::Attribute("Karmaenergie", "KaP"), 0, 127);
        character.add_stat(Stat::Attribute("Seelenenkraft", "SK"), 0, 127);
        character.add_stat(Stat::Attribute("Zähigkeit", "ZK"), 0, 127);
        character.add_stat(Stat::Attribute("Ausweichen", "AW"), 0, 127);
        character.add_stat(Stat::Attribute("Initiative", "INI"), 0, 127);
        character.add_stat(Stat::Attribute("Geschwindigkeit", "GS"), 0, 127);
        character.add_stat(Stat::Attribute("Schicksalspunkte", "Schips"), -127, 0);
        sheet.add_category(character);

        let mut attributes = StatCategory::new("Attribute");
        attributes.add_stat(Stat::Attribute("Mut", "MU"), 8, 19);
        attributes.add_stat(Stat::Attribute("Klugheit", "KL"), 8, 19);
        attributes.add_stat(Stat::Attribute("Intuition", "IN"), 8, 19);
        attributes.add_stat(Stat::Attribute("Charisma", "CH"), 8, 19);
        attributes.add_stat(Stat::Attribute("Fingerfertigkeit", "FF"), 8, 19);
        attributes.add_stat(Stat::Attribute("Gewandheit", "GE"), 8, 19);
        attributes.add_stat(Stat::Attribute("Konstitution", "KO"), 8, 19);
        attributes.add_stat(Stat::Attribute("Körperkraft", "KK"), 8, 19);
        sheet.add_category(attributes);

        let mut weapons = StatCategory::new("Kampftechnik");
        weapons.add_stat(Stat::Ability("Armbrüste", vec!["FF"]), -1, 25);
        weapons.add_stat(Stat::Calculated("Armbrüste - Fernkampf"), 0, 0);
        weapons.add_stat(Stat::Calculated("Armbrüste - Parade"), 0, 0);
        weapons.add_stat(Stat::Ability("Bögen", vec!["FF"]), -1, 25);
        weapons.add_stat(Stat::Calculated("Bögen - Fernkampf"), 0, 0);
        weapons.add_stat(Stat::Calculated("Bögen - Parade"), 0, 0);
        weapons.add_stat(Stat::Ability("Dolche", vec!["GE"]), -1, 25);
        weapons.add_stat(Stat::Calculated("Dolche - Attacke"), 0, 0);
        weapons.add_stat(Stat::Calculated("Dolche - Parade"), 0, 0);
        weapons.add_stat(Stat::Ability("Fechtwaffen", vec!["GE"]), -1, 25);
        weapons.add_stat(Stat::Calculated("Fechtwaffen - Attacke"), 0, 0);
        weapons.add_stat(Stat::Calculated("Fechtwaffen - Parade"), 0, 0);
        weapons.add_stat(Stat::Ability("Hiebwaffen", vec!["KK"]), -1, 25);
        weapons.add_stat(Stat::Calculated("Hiebwaffen - Attacke"), 0, 0);
        weapons.add_stat(Stat::Calculated("Hiebwaffen - Parade"), 0, 0);
        weapons.add_stat(Stat::Ability("Kettenwaffen", vec!["KK"]), -1, 25);
        weapons.add_stat(Stat::Calculated("Kettenwaffen - Attacke"), 0, 0);
        weapons.add_stat(Stat::Calculated("Kettenwaffen - Parade"), 0, 0);
        weapons.add_stat(Stat::Ability("Lanzen", vec!["KK"]), -1, 25);
        weapons.add_stat(Stat::Calculated("Lanzen - Attacke"), 0, 0);
        weapons.add_stat(Stat::Calculated("Lanzen - Parade"), 0, 0);
        weapons.add_stat(Stat::Ability("Raufen", vec!["GE", "KK"]), -1, 25);
        weapons.add_stat(Stat::Calculated("Raufen - Attacke"), 0, 0);
        weapons.add_stat(Stat::Calculated("Raufen - Parade"), 0, 0);
        weapons.add_stat(Stat::Ability("Schilde", vec!["KK"]), -1, 25);
        weapons.add_stat(Stat::Calculated("Schilde - Attacke"), 0, 0);
        weapons.add_stat(Stat::Calculated("Schilde - Parade"), 0, 0);
        weapons.add_stat(Stat::Ability("Schwerter", vec!["GE", "KK"]), -1, 25);
        weapons.add_stat(Stat::Calculated("Schwerter - Attacke"), 0, 0);
        weapons.add_stat(Stat::Calculated("Schwerter - Parade"), 0, 0);
        weapons.add_stat(Stat::Ability("Stangenwaffen", vec!["GE", "KK"]), -1, 25);
        weapons.add_stat(Stat::Calculated("Stangenwaffen - Attacke"), 0, 0);
        weapons.add_stat(Stat::Calculated("Stangenwaffen - Parade"), 0, 0);
        weapons.add_stat(Stat::Ability("Wurfwaffen", vec!["FF"]), -1, 25);
        weapons.add_stat(Stat::Calculated("Wurfwaffen - Fernkampf"), 0, 0);
        weapons.add_stat(Stat::Calculated("Wurfwaffen - Parade"), 0, 0);
        weapons.add_stat(Stat::Ability("Zweihandhiebwaffen", vec!["KK"]), -1, 25);
        weapons.add_stat(Stat::Calculated("Zweihandhiebwaffen - Attacke"), 0, 0);
        weapons.add_stat(Stat::Calculated("Zweihandhiebwaffen - Parade"), 0, 0);
        weapons.add_stat(Stat::Ability("Zweihandschwerter", vec!["KK"]), -1, 25);
        weapons.add_stat(Stat::Calculated("Zweihandschwerter - Attacke"), 0, 0);
        weapons.add_stat(Stat::Calculated("Zweihandschwerter - Parade"), 0, 0);
        sheet.add_category(weapons);

        let mut physical = StatCategory::new("Körpertalente");
        physical.add_stat(Stat::Ability("Schleichen", vec!["MU", "IN", "GE"]), -1, 25);
        physical.add_stat(
            Stat::Ability("Selbstbeherrschung", vec!["MU", "KO", "KK"]),
            -1,
            25,
        );
        physical.add_stat(
            Stat::Ability("Sinnesschärfe", vec!["KL", "IN", "GE"]),
            -1,
            25,
        );
        physical.add_stat(
            Stat::Ability("Taschendiebstahl", vec!["KL", "IN", "FF"]),
            -1,
            25,
        );
        physical.add_stat(Stat::Ability("Klettern", vec!["GE", "GE", "KK"]), -1, 20);
        physical.add_stat(Stat::Ability("Athletik", vec!["GE", "GE", "IN"]), -1, 20);
        physical.add_stat(Stat::Ability("Springen", vec!["GE", "GE", "KK"]), -1, 20);
        physical.add_stat(
            Stat::Ability("(Ent-)Fesseln", vec!["GE", "KK", "FF"]),
            -1,
            25,
        );
        sheet.add_category(physical);

        sheet
    }
}
