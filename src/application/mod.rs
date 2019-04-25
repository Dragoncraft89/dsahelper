use qt_widgets::qt_core;

use qt_widgets::cpp_utils::CppBox;
use qt_widgets::cpp_utils::StaticCast;

use qt_core::variant::Variant;

use qt_core::string_list_model::StringListModel;
use qt_core::abstract_item_model::AbstractItemModel;

use qt_widgets::widget::Widget;
use qt_widgets::list_view::ListView;

use qt_widgets::input_dialog::InputDialog;

use crate::qt_bind::find_child;

pub trait PenAndPaperCalendar {
}

pub trait Player {
    fn name(&self) -> &String;
    fn set_name(&mut self, name: String);
}

pub trait PenAndPaperBackend {
    fn calendar(&mut self) -> &mut PenAndPaperCalendar;

    fn add_player(&mut self, name: String) -> &Player;
    fn get_player(&mut self, pos: usize) -> &mut Player;
    fn remove_player(&mut self, pos: usize);
}

pub struct TestCalendar {
}

pub struct TestPlayer {
    _name: String
}

impl Player for TestPlayer {
    fn name(&self) -> &String {
        &self._name
    }

    fn set_name(&mut self, name: String) {
        self._name = name
    }
}

impl PenAndPaperCalendar for TestCalendar {
}

pub struct Test {
    cal: TestCalendar,
    players: Vec<TestPlayer>
}

impl Test {
    fn new() -> Test {
        Test { cal: TestCalendar {}, players: Vec::new() }
    }
}

impl PenAndPaperBackend for Test {
    fn calendar(&mut self) -> &mut PenAndPaperCalendar {
        &mut self.cal
    }

    fn add_player(&mut self, name: String) -> &Player {
        self.players.push(TestPlayer {_name: name});
        self.players.last().unwrap()
    }
    
    fn get_player(&mut self, pos: usize) -> &mut Player {
        &mut self.players[pos]
    }
    
    fn remove_player(&mut self, pos: usize) {
        self.players.remove(pos);
    }
}

pub struct Application {
    player_list_model: CppBox<StringListModel>,
    
    backend: Option<Box<PenAndPaperBackend>>,
    main_window: *mut Widget
}

impl Application {
    pub fn new(main_window: *mut Widget) -> Application {
        let app = Application { player_list_model: StringListModel::new(()), backend: None, main_window: main_window };

        let listview = find_child(main_window, "players").unwrap() as *mut ListView;
        unsafe {
            (*listview).set_model(StringListModel::static_cast_mut(Box::leak(Box::from_raw(app.player_list_model.as_mut_ptr()))) as *mut AbstractItemModel);
        }
        
        app
    }

    pub fn new_file(&mut self) {
        println!("New stub");
        self.backend = Some(Box::new(Test::new()));
    }

    pub fn open(&mut self) {
        println!("Open stub");
    }

    pub fn close(&self) {
        qt_core::core_application::CoreApplication::quit();
    }

    pub fn options(&mut self) {
        println!("Options stub");
    }

    pub fn selection_changed(&mut self) {
        println!("Selection changed stub");
    }

    pub fn add_player(&mut self) {
        if let Some(var) = &mut self.backend {
            let mut ok = false;
            let name = unsafe { InputDialog::get_text((self.main_window, &"Spieler erstellen".into(), &"Spielername eingeben:".into(), qt_widgets::line_edit::EchoMode::Normal, &"".into(), &mut ok as *mut bool)) };

            if ok {
                let player = var.add_player(name.to_std_string());
                
                let model = &mut (*self.player_list_model);
                let row_count = model.row_count(());
                model.insert_row(row_count);
                let index = model.index(row_count);
                model.set_data((&index, &Variant::new0(&qt_core::string::String::from(player.name().as_str()))));
            }
        }
    }

    pub fn edit_player(&mut self) {
        if let Some(var) = &mut self.backend {
            let listview = find_child(self.main_window, "players").unwrap() as *mut ListView;
            let indexes = unsafe {(*(*listview).selection_model()).selected_indexes()};
            for i in 0..indexes.size() {
                let pos = indexes.at(i).row() as usize;
                let player = var.get_player(pos);
                let mut ok = false;
                let name = unsafe { InputDialog::get_text((self.main_window, &"Spieler bearbeiten".into(), &"Spielername eingeben:".into(), qt_widgets::line_edit::EchoMode::Normal, &(**player.name()).into(), &mut ok as *mut bool)) };
                
                if ok {
                    let model = &mut (*self.player_list_model);

                    player.set_name(name.to_std_string());
                    model.set_data((indexes.at(i), &Variant::new0(&qt_core::string::String::from(player.name().as_str()))));
                }
            }
        }
    }

    pub fn remove_player(&mut self) {
        if let Some(var) = &mut self.backend {
            let listview = find_child(self.main_window, "players").unwrap() as *mut ListView;
            let indexes = unsafe {(*(*listview).selection_model()).selected_indexes()};
            
            for i in 0..indexes.size() {
                var.remove_player(indexes.at(i).row() as usize);

                self.player_list_model.remove_rows((indexes.at(i).row(), 1));
            }
        }
    }
}
