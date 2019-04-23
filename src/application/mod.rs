extern crate serde;

use qt_widgets::qt_core;

use serde::{Serialize, Deserialize};

pub trait PenAndPaperCalendar {
}

pub trait PenAndPaperBackend<'a>: Serialize + Deserialize<'a> {
    fn calendar() -> PenAndPaperCalendar;
}

pub struct Application {
}

impl Application {
    pub fn new() -> Application {
        Application { }
    }

    pub fn new_file(&mut self) {
        println!("New stub");
    }

    pub fn open(&mut self) {
        println!("Open stub");
    }

    pub fn close(&mut self) {
        qt_core::core_application::CoreApplication::quit();
    }

    pub fn options(&mut self) {
        println!("Options stub");
    }
}
