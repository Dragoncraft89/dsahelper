#[macro_use]
extern crate lazy_static;
extern crate qt_widgets;

mod application;
mod qt_bind;

use qt_widgets::qt_core;

use qt_widgets::cpp_utils::StaticCast;

use qt_core::string::String;
use qt_core::file::File;
use qt_core::flags::Flags;
use qt_core::io_device::{OpenModeFlag, IODevice};

use qt_widgets::list_view::ListView;

use qt_ui_tools::ui_loader::UiLoader;

use application::Application;
use crate::qt_bind::*;

macro_rules! version {
    () => {
        format!("{}.{}.{}",
            env!("CARGO_PKG_VERSION_MAJOR"),
            env!("CARGO_PKG_VERSION_MINOR"),
            env!("CARGO_PKG_VERSION_PATCH"))
    }
}

fn main() {
    qt_widgets::application::Application::create_and_exit(|_app| {
        qt_core::core_application::CoreApplication::set_application_name(&String::from("DSAHelper"));
        qt_core::core_application::CoreApplication::set_application_version(&String::from(version!().as_str()));

        let s = String::from("ui/main_window.ui");
        let mut file = File::new(&s);
        file.open(Flags::from_enum(OpenModeFlag::ReadOnly));
        let mut loader = UiLoader::new();
        let main_window;
        unsafe {
            main_window = loader.load(File::static_cast_mut(&mut file) as *mut IODevice);
            (*main_window).show();
        }

        let mut backend = Application::new(main_window);
        
        connect!(find_child(main_window, "new_file").unwrap(), SIGNAL!("triggered()"), &mut backend, Application, Application::new_file);
        connect!(find_child(main_window, "open").unwrap(), SIGNAL!("triggered()"), &mut backend, Application, Application::open);
        connect!(find_child(main_window, "close").unwrap(), SIGNAL!("triggered()"), &mut backend, Application, Application::close);
        connect!(find_child(main_window, "options").unwrap(), SIGNAL!("triggered()"), &mut backend, Application, Application::options);

        connect!(as_object((*(find_child(main_window, "players").unwrap() as *mut ListView)).selection_model()), SIGNAL!("currentChanged(const QModelIndex &, const QModelIndex &)"), &mut backend, Application, Application::selection_changed);
        
        connect!(find_child(main_window, "add_player").unwrap(), SIGNAL!("pressed()"), &mut backend, Application, Application::add_player);
        connect!(find_child(main_window, "edit_player").unwrap(), SIGNAL!("pressed()"), &mut backend, Application, Application::edit_player);
        connect!(find_child(main_window, "remove_player").unwrap(), SIGNAL!("pressed()"), &mut backend, Application, Application::remove_player);

        qt_widgets::application::Application::exec()
    })
}
