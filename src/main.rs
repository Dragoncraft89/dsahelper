#[macro_use]
extern crate lazy_static;
extern crate qt_widgets;

mod qt_bind;
mod application;

use qt_widgets::qt_core;

use qt_core::string::String;

use qt_widgets::list_view::ListView;

use crate::qt_bind::*;
use application::backend::TimeUnits;
use application::Application;

macro_rules! version {
    () => {
        format!(
            "{}.{}.{}",
            env!("CARGO_PKG_VERSION_MAJOR"),
            env!("CARGO_PKG_VERSION_MINOR"),
            env!("CARGO_PKG_VERSION_PATCH")
        )
    };
}

fn main() {
    qt_widgets::application::Application::create_and_exit(|_app| {
        qt_widgets::qt_gui::font_database::FontDatabase::add_application_font(&String::from(
            "fonts/fontawesome.otf",
        ));
        qt_core::core_application::CoreApplication::set_application_name(&String::from(
            "DSAHelper",
        ));
        qt_core::core_application::CoreApplication::set_application_version(&String::from(
            version!().as_str(),
        ));

        let main_window = load("ui/main_window.ui");
        unsafe {
            (*main_window).show();
        }

        let mut backend = Application::new(main_window);

        connect!(
            find_child(main_window, "new_file").unwrap(),
            SIGNAL!("triggered()"),
            &mut backend,
            Application,
            Application::new_file
        );
        connect!(
            find_child(main_window, "open").unwrap(),
            SIGNAL!("triggered()"),
            &mut backend,
            Application,
            Application::open
        );
        connect!(
            find_child(main_window, "close").unwrap(),
            SIGNAL!("triggered()"),
            &mut backend,
            Application,
            Application::close
        );
        connect!(
            find_child(main_window, "options").unwrap(),
            SIGNAL!("triggered()"),
            &mut backend,
            Application,
            Application::options
        );

        let selection_model = unsafe {
            (*(find_child(main_window, "players").unwrap() as *mut ListView)).selection_model()
        };
        connect!(
            as_object(selection_model),
            SIGNAL!("selectionChanged(const QItemSelection &, const QItemSelection &)"),
            &mut backend,
            Application,
            Application::selection_changed,
            selection_model,
            *mut qt_core::item_selection_model::ItemSelectionModel
        );

        connect!(
            find_child(main_window, "add_player").unwrap(),
            SIGNAL!("pressed()"),
            &mut backend,
            Application,
            Application::add_player
        );
        connect!(
            find_child(main_window, "edit_player").unwrap(),
            SIGNAL!("pressed()"),
            &mut backend,
            Application,
            Application::edit_player
        );
        connect!(
            find_child(main_window, "remove_player").unwrap(),
            SIGNAL!("pressed()"),
            &mut backend,
            Application,
            Application::remove_player
        );

        connect!(
            find_child(main_window, "set_date").unwrap(),
            SIGNAL!("pressed()"),
            &mut backend,
            Application,
            Application::set_date
        );

        connect!(
            find_child(main_window, "min15").unwrap(),
            SIGNAL!("pressed()"),
            &mut backend,
            Application,
            Application::add_time,
            TimeUnits::Minutes(15),
            TimeUnits
        );
        connect!(
            find_child(main_window, "min30").unwrap(),
            SIGNAL!("pressed()"),
            &mut backend,
            Application,
            Application::add_time,
            TimeUnits::Minutes(30),
            TimeUnits
        );
        connect!(
            find_child(main_window, "hour1").unwrap(),
            SIGNAL!("pressed()"),
            &mut backend,
            Application,
            Application::add_time,
            TimeUnits::Hours(1),
            TimeUnits
        );
        connect!(
            find_child(main_window, "day1").unwrap(),
            SIGNAL!("pressed()"),
            &mut backend,
            Application,
            Application::add_time,
            TimeUnits::Days(1),
            TimeUnits
        );
        connect!(
            find_child(main_window, "week1").unwrap(),
            SIGNAL!("pressed()"),
            &mut backend,
            Application,
            Application::add_time,
            TimeUnits::Weeks(1),
            TimeUnits
        );
        connect!(
            find_child(main_window, "month1").unwrap(),
            SIGNAL!("pressed()"),
            &mut backend,
            Application,
            Application::add_time,
            TimeUnits::Months(1),
            TimeUnits
        );

        connect!(
            find_child(main_window, "next_day").unwrap(),
            SIGNAL!("pressed()"),
            &mut backend,
            Application,
            Application::next_day
        );
        connect!(
            find_child(main_window, "next_evening").unwrap(),
            SIGNAL!("pressed()"),
            &mut backend,
            Application,
            Application::next_evening
        );

        qt_widgets::application::Application::exec()
    })
}
