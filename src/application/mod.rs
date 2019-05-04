use qt_widgets::qt_core;

use qt_widgets::cpp_utils::CppBox;
use qt_widgets::cpp_utils::StaticCast;

use qt_core::variant::Variant;
use qt_core::object::Object;

use qt_core::file::File;
use qt_core::flags::Flags;
use qt_core::io_device::{OpenModeFlag, IODevice};

use qt_core::string_list_model::StringListModel;
use qt_core::abstract_item_model::AbstractItemModel;

use qt_widgets::widget::Widget;
use qt_widgets::list_view::ListView;
use qt_widgets::label::Label;
use qt_widgets::spin_box::SpinBox;
use qt_widgets::line_edit::EchoMode;

use qt_widgets::v_box_layout::VBoxLayout;

use qt_widgets::input_dialog::InputDialog;

use qt_ui_tools::ui_loader::UiLoader;

use crate::qt_bind;
use qt_bind::{find_child, find_child_layout, delete, as_object};

mod backend;
mod dsa;

use backend::*;
use dsa::*;

pub struct Application {
    player_list_model: CppBox<StringListModel>,
    selected_player_index: Option<usize>,
    
    backend: Option<Box<PenAndPaperBackend>>,
    main_window: *mut Widget
}

impl Application {
    pub fn new(main_window: *mut Widget) -> Application {
        let app = Application { player_list_model: StringListModel::new(()), backend: None, main_window: main_window, selected_player_index: None };

        let listview = find_child(main_window, "players").unwrap() as *mut ListView;
        unsafe {
            let boxed_model = Box::from_raw(app.player_list_model.as_mut_ptr());
            let model = StringListModel::static_cast_mut(Box::leak(boxed_model)) as *mut AbstractItemModel;
            (*listview).set_model(model);
        }
        
        app
    }

    fn build_character_sheet(&mut self) {
        if let Some(backend) = &mut self.backend {
            let layout = unsafe {&mut *(find_child(self.main_window, "character_sheet").unwrap() as *mut VBoxLayout) };

            let mut child = layout.take_at(0);
            while !child.is_null() {
                delete(child as *mut Object);
                child = layout.take_at(0);
            }
            
            let header_filename = qt_core::string::String::from("ui/character_sheet/header.ui");
            let mut header = File::new(&header_filename);
            header.open(Flags::from_enum(OpenModeFlag::ReadOnly));

            let attribute_filename = qt_core::string::String::from("ui/character_sheet/attribute.ui");
            let mut attribute = File::new(&attribute_filename);
            attribute.open(Flags::from_enum(OpenModeFlag::ReadOnly));

            let ability_filename = qt_core::string::String::from("ui/character_sheet/ability.ui");
            let mut ability = File::new(&ability_filename);
            ability.open(Flags::from_enum(OpenModeFlag::ReadOnly));
            
            let mut loader = UiLoader::new();

            for category in backend.character_sheet().categories() {
                unsafe {
                    let widget = loader.load(File::static_cast_mut(&mut header) as *mut IODevice);
                    (*as_object(widget)).set_object_name(&qt_core::string::String::from(category.name.as_str()));
                    (*(find_child(widget, "name").unwrap() as *mut Label)).set_text(&qt_core::string::String::from(format!("<b>{}</b>", category.name).as_str()));
                    (*(find_child(widget, "remaining").unwrap() as *mut Label)).set_text(&qt_core::string::String::from(""));
                    
                    layout.add_widget(widget);
                }
                header.reset();

                for stat in &category.stats {
                    let widget = match &stat.stat {
                        Stat::Attribute(name, short) => unsafe {
                            let widget = loader.load(File::static_cast_mut(&mut attribute) as *mut IODevice);
                            attribute.reset();
                            (*as_object(widget)).set_object_name(&qt_core::string::String::from(short.as_str()));
                            (*(find_child(widget, "identifier").unwrap() as *mut Label)).set_text(&qt_core::string::String::from(format!("{} ({})", name, short).as_str()));
                            (*(find_child(widget, "calculated").unwrap() as *mut Label)).set_text(&qt_core::string::String::from("0"));
                            
                            widget
                        },
                        Stat::Ability(name, stats) => unsafe {
                            let widget = loader.load(File::static_cast_mut(&mut ability) as *mut IODevice);
                            ability.reset();
                            (*as_object(widget)).set_object_name(&qt_core::string::String::from(name.as_str()));
                            (*(find_child(widget, "identifier").unwrap() as *mut Label)).set_text(&qt_core::string::String::from(name.as_str()));
                            (*(find_child(widget, "stats").unwrap() as *mut Label)).set_text(&qt_core::string::String::from(stats.join(" ").as_str()));
                            (*(find_child(widget, "calculated").unwrap() as *mut Label)).set_text(&qt_core::string::String::from("0"));

                            widget
                        }
                    };

                    let value = find_child(widget, "value").unwrap();
                    unsafe {
                        (*(value as *mut SpinBox)).set_range(stat.min as i32, stat.max as i32);
                    }
                    connect!(value, SIGNAL!("valueChanged(int)"), &mut *self, Application, Application::change_value, (category.name.clone(), stat.stat.clone(), value as *mut SpinBox), (String, Stat, *mut SpinBox));

                    unsafe {
                        layout.add_widget(widget);
                    }
                }
                layout.add_spacing(25);
            }
        }
    }

    fn update_character_sheet(&mut self) {
        if let (Some(backend), Some(player_index)) = (&mut self.backend, self.selected_player_index) {
            let layout = unsafe {&mut *(find_child(self.main_window, "character_sheet").unwrap() as *mut VBoxLayout) };
            let sheet = backend.character_sheet();
            for category in sheet.categories() {
                let player = backend.get_player(player_index);
                let cat = find_child_layout(layout, category.name.as_str()).unwrap() as *mut qt_widgets::widget::Widget;
                unsafe {
                    let str = qt_core::string::String::number0(category.get_remaining_points(&sheet, player) as i32);
                    (*(find_child(cat, "remaining").unwrap() as *mut Label)).set_text(&str);
                }

                for stat in &category.stats {
                    let widget = match &stat.stat {
                        Stat::Attribute(_, short) => find_child_layout(layout, short.as_str()).unwrap(),
                        Stat::Ability(name, _) => find_child_layout(layout, name.as_str()).unwrap()
                    } as *mut qt_widgets::widget::Widget;

                    let val = player.get_value(&stat.stat);
                    unsafe {
                        (*(find_child(widget, "value").unwrap() as *mut SpinBox)).set_value(val as i32);
                    }
                }
            }
        }
    }

    pub fn change_value(&mut self, arg: &mut (String, Stat, *mut SpinBox)) {
        if let (Some(backend), Some(player_index)) = (&mut self.backend, self.selected_player_index) {
            let sheet = backend.character_sheet();

            let player = backend.get_player(player_index);
            let value;
            unsafe {
                value = sheet.validate_value(player, sheet.get_category(&arg.0).unwrap(), &arg.1, (*arg.2).value() as i8);
                (*as_object(arg.2)).block_signals(true);
            }

            player.set_value(arg.1.clone(), value);
            self.update_character_sheet();

            unsafe {
                (*as_object(arg.2)).block_signals(false);
            }
        }
    }
    
    pub fn new_file(&mut self) {
        println!("New stub");
        self.backend = Some(Box::new(DSABackend::new()));
        unsafe {
            (*(find_child(self.main_window, "centralwidget").unwrap() as *mut qt_widgets::widget::Widget)).set_enabled(true);
        }

        self.build_character_sheet();
        let row_count = self.player_list_model.row_count(());
        self.player_list_model.remove_rows((0, row_count));
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

    pub fn selection_changed(&mut self, model: &mut *mut qt_core::item_selection_model::ItemSelectionModel) {
        let selections = unsafe {(**model).selected_indexes()};

        self.selected_player_index = match selections.size() {
            0 => None,
            _ => Some(selections.at(0).row() as usize)
        };
        
        unsafe {
            (*(find_child(self.main_window, "player_data").unwrap() as *mut qt_widgets::widget::Widget)).set_enabled(selections.size() == 1);
        }

        self.update_character_sheet();
    }

    pub fn add_player(&mut self) {
        if let Some(var) = &mut self.backend {
            let mut ok = false;
            let name = unsafe {
                let title = "Spieler erstellen".into();
                let label = "Spielername eingeben".into();
                let text = "".into();
                InputDialog::get_text((self.main_window, &title, &label, EchoMode::Normal, &text, &mut ok as *mut bool))
            };

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
                let name = unsafe {
                    let title = "Spieler bearbeiten".into();
                    let label = "Spielername eingeben:".into();
                    let text = (**player.name()).into();
                    InputDialog::get_text((self.main_window, &title, &label, EchoMode::Normal, &text, &mut ok as *mut bool))
                };
                
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
