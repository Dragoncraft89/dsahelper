use qt_widgets::qt_core;

use qt_widgets::cpp_utils::CppBox;
use qt_widgets::cpp_utils::StaticCast;

use qt_core::object::Object;
use qt_core::variant::Variant;

use qt_core::abstract_item_model::AbstractItemModel;
use qt_core::string_list_model::StringListModel;

use qt_widgets::combo_box::ComboBox;
use qt_widgets::label::Label;
use qt_widgets::layout::Layout;
use qt_widgets::line_edit::EchoMode;
use qt_widgets::list_view::ListView;
use qt_widgets::push_button::PushButton;
use qt_widgets::spin_box::SpinBox;
use qt_widgets::widget::Widget;

use qt_widgets::v_box_layout::VBoxLayout;

use qt_widgets::input_dialog::InputDialog;

use qt_widgets::button_group::ButtonGroup;
use qt_widgets::dialog::Dialog;

use crate::qt_bind;
use qt_bind::{as_object, delete, find_child, find_child_layout, load};

pub mod backend;
mod dsa;

use backend::*;
use dsa::*;

pub struct Application {
    player_list_model: CppBox<StringListModel>,
    selected_player_index: Option<usize>,

    backend: Option<Box<PenAndPaperBackend>>,
    main_window: *mut Widget,
}

impl Application {
    pub fn new(main_window: *mut Widget) -> Application {
        let app = Application {
            player_list_model: StringListModel::new(()),
            backend: None,
            main_window: main_window,
            selected_player_index: None,
        };

        let listview = find_child(main_window, "players").unwrap() as *mut ListView;
        unsafe {
            let boxed_model = Box::from_raw(app.player_list_model.as_mut_ptr());
            let model =
                StringListModel::static_cast_mut(Box::leak(boxed_model)) as *mut AbstractItemModel;
            (*listview).set_model(model);
        }

        app
    }

    fn build_character_sheet(&mut self) {
        if let Some(backend) = &mut self.backend {
            let layout = unsafe {
                &mut *(find_child(self.main_window, "character_sheet").unwrap() as *mut VBoxLayout)
            };

            let mut child = layout.take_at(0);
            while !child.is_null() {
                delete(child as *mut Object);
                child = layout.take_at(0);
            }

            let character_sheet = backend.character_sheet();

            for category in character_sheet.categories() {
                unsafe {
                    let widget = load("ui/character_sheet/header.ui");
                    let name = &mut *(find_child(widget, "name").unwrap() as *mut Label);
                    (*as_object(widget)).set_object_name(&qt_string!(category.name));
                    name.set_text(&qt_string!(category.name));

                    layout.add_widget(widget);
                }

                for entry in &category.entries {
                    match entry {
                        CategoryEntry::Modifier(modifier) => unsafe {
                            let widget = load("ui/character_sheet/modifier.ui");
                            (*as_object(widget)).set_object_name(&qt_string!(modifier.name));
                            let name = &mut *(find_child(widget, "name").unwrap() as *mut Label);
                            let values =
                                &mut *(find_child(widget, "values").unwrap() as *mut ComboBox);

                            name.set_text(&qt_string!(modifier.name));

                            connect!(
                                as_object(values),
                                SIGNAL!("currentIndexChanged(int)"),
                                self,
                                Application,
                                Application::change_modifier,
                                (modifier.name.to_string(), values),
                                (String, *mut ComboBox)
                            );

                            layout.add_widget(widget);
                        },
                        CategoryEntry::Stat(stat) => {
                            let widget = match &stat.stat {
                                Stat::Attribute(name, short) => unsafe {
                                    let widget = load("ui/character_sheet/attribute.ui");
                                    let identifier = &mut *(find_child(widget, "identifier")
                                        .unwrap()
                                        as *mut Label);
                                    let calculated = &mut *(find_child(widget, "calculated")
                                        .unwrap()
                                        as *mut Label);
                                    (*as_object(widget)).set_object_name(&qt_string!(*short));
                                    identifier
                                        .set_text(&qt_string!(format!("{} ({})", name, short)));
                                    calculated.set_text(&qt_string!("0"));

                                    widget
                                },
                                Stat::Ability(name, stats) => unsafe {
                                    let widget = load("ui/character_sheet/ability.ui");
                                    let identifier = &mut *(find_child(widget, "identifier")
                                        .unwrap()
                                        as *mut Label);
                                    let stats_label =
                                        &mut *(find_child(widget, "stats").unwrap() as *mut Label);
                                    let calculated = &mut *(find_child(widget, "calculated")
                                        .unwrap()
                                        as *mut Label);
                                    (*as_object(widget)).set_object_name(&qt_string!(*name));
                                    identifier.set_text(&qt_string!(*name));
                                    stats_label.set_text(&qt_string!(stats.join(" ")));
                                    calculated.set_text(&qt_string!("0"));

                                    widget
                                },
                            };

                            let value = find_child(widget, "value").unwrap();
                            unsafe {
                                (*(value as *mut SpinBox)).set_range(stat.min, stat.max);
                                connect!(
                                    value,
                                    SIGNAL!("valueChanged(int)"),
                                    &mut *self,
                                    Application,
                                    Application::change_value,
                                    (
                                        category.name.to_string(),
                                        stat.stat.clone(),
                                        value as *mut SpinBox
                                    ),
                                    (String, Stat, *mut SpinBox)
                                );
                            }

                            unsafe {
                                layout.add_widget(widget);
                            }
                        }
                    }
                }
                layout.add_spacing(25);
            }
        }
    }

    fn update_character_sheet(&mut self) {
        if let (Some(backend), Some(player_index)) = (&mut self.backend, self.selected_player_index)
        {
            let layout = unsafe {
                &mut *(find_child(self.main_window, "character_sheet").unwrap() as *mut VBoxLayout)
            };
            let sheet = backend.character_sheet();

            for category in sheet.categories() {
                let player = backend.get_player(player_index);

                for entry in &category.entries {
                    match entry {
                        CategoryEntry::Modifier(modifier) => {
                            let values = modifier.get_values(player);
                            let index = match values.iter().position(|x| {
                                x.as_ref().name()
                                    == player.get_modifier(&modifier.name.to_string()).name()
                            }) {
                                Some(x) => x,
                                None => {
                                    player.set_modifier(
                                        modifier.name.to_string(),
                                        modifier.get_values(player).swap_remove(0),
                                    );
                                    0
                                }
                            };

                            unsafe {
                                let entry = find_child_layout(layout, modifier.name).unwrap()
                                    as *mut Layout;
                                let combobox =
                                    &mut (*(find_child(entry, "values").unwrap() as *mut ComboBox));
                                (*as_object(combobox)).block_signals(true);
                                while combobox.count() > 0 {
                                    combobox.remove_item(0);
                                }
                                for value in values {
                                    combobox.add_item(&qt_string!(value.name()));
                                }
                                combobox.set_current_index(index as i32);
                                (*as_object(combobox)).block_signals(false);
                            }
                        }
                        CategoryEntry::Stat(stat) => {
                            let widget = match &stat.stat {
                                Stat::Attribute(_, short) => {
                                    find_child_layout(layout, short).unwrap()
                                }
                                Stat::Ability(name, _) => find_child_layout(layout, name).unwrap(),
                            } as *mut Layout;
                            let calculated = sheet.calc_value(player, &category, &stat.stat);
                            let val = player.get_value(&stat.stat);
                            unsafe {
                                (*(find_child(widget, "calculated").unwrap() as *mut Label))
                                    .set_text(&qt_string!(calculated));
                                (*(find_child(widget, "value").unwrap() as *mut SpinBox))
                                    .set_value(val);
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn change_modifier(&mut self, (name, widget): &mut (String, *mut ComboBox)) {
        if let (Some(backend), Some(player_index)) = (&mut self.backend, self.selected_player_index)
        {
            let mut sheet = backend.character_sheet();

            let player = backend.get_player(player_index);

            let index = unsafe { (**widget).current_index() as usize };
            let value = match sheet
                .categories_mut()
                .iter_mut()
                .map(|x| {
                    x.entries.iter_mut().find(|y| {
                        if let CategoryEntry::Modifier(m) = y {
                            &m.name == name
                        } else {
                            false
                        }
                    })
                })
                .find(|x| x.is_some())
                .unwrap()
                .unwrap()
            {
                CategoryEntry::Modifier(modifier) => modifier.get_values(player).swap_remove(index),
                _ => panic!("Application::change_value: Expected modifier as type"),
            };
            player.set_modifier(name.to_string(), value);
            self.update_character_sheet();
        }
    }

    pub fn change_value(&mut self, arg: &mut (String, Stat, *mut SpinBox)) {
        if let (Some(backend), Some(player_index)) = (&mut self.backend, self.selected_player_index)
        {
            let player = backend.get_player(player_index);
            let value;
            unsafe {
                value = (*arg.2).value();
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
            (*(find_child(self.main_window, "centralwidget").unwrap() as *mut Widget))
                .set_enabled(true);
        }

        self.build_character_sheet();
        self.update_time();
        self.update_date();
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

    pub fn selection_changed(
        &mut self,
        model: &mut *mut qt_core::item_selection_model::ItemSelectionModel,
    ) {
        let selections = unsafe { (**model).selected_indexes() };

        self.selected_player_index = match selections.size() {
            0 => None,
            _ => Some(selections.at(0).row() as usize),
        };

        unsafe {
            (*(find_child(self.main_window, "player_data").unwrap() as *mut Widget))
                .set_enabled(selections.size() == 1);
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
                InputDialog::get_text((
                    self.main_window,
                    &title,
                    &label,
                    EchoMode::Normal,
                    &text,
                    &mut ok as *mut bool,
                ))
            };

            if ok {
                let player = var.add_player(name.to_std_string());

                let model = &mut (*self.player_list_model);
                let row_count = model.row_count(());
                model.insert_row(row_count);
                let index = model.index(row_count);
                model.set_data((&index, &Variant::new0(&qt_string!(player.name()))));
            }
        }
    }

    pub fn edit_player(&mut self) {
        if let Some(var) = &mut self.backend {
            let listview = find_child(self.main_window, "players").unwrap() as *mut ListView;
            let indexes = unsafe { (*(*listview).selection_model()).selected_indexes() };
            for i in 0..indexes.size() {
                let pos = indexes.at(i).row() as usize;
                let player = var.get_player(pos);
                let mut ok = false;
                let name = unsafe {
                    let title = "Spieler bearbeiten".into();
                    let label = "Spielername eingeben:".into();
                    let text = (**player.name()).into();
                    InputDialog::get_text((
                        self.main_window,
                        &title,
                        &label,
                        EchoMode::Normal,
                        &text,
                        &mut ok as *mut bool,
                    ))
                };

                if ok {
                    let model = &mut (*self.player_list_model);

                    player.set_name(name.to_std_string());
                    model.set_data((indexes.at(i), &Variant::new0(&qt_string!(player.name()))));
                }
            }
        }
    }

    pub fn remove_player(&mut self) {
        if let Some(var) = &mut self.backend {
            let listview = find_child(self.main_window, "players").unwrap() as *mut ListView;
            let indexes = unsafe { (*(*listview).selection_model()).selected_indexes() };

            for i in 0..indexes.size() {
                var.remove_player(indexes.at(i).row() as usize);

                self.player_list_model.remove_rows((indexes.at(i).row(), 1));
            }
        }
    }

    pub fn update_time(&mut self) {
        if let Some(backend) = &mut self.backend {
            let time_label = find_child(self.main_window, "time").unwrap() as *mut Label;

            let (hour, minute) = backend.calendar().get_time();
            unsafe {
                (*time_label).set_text(&qt_string!(format!("{:02}:{:02}", hour, minute)));
            }
        }
    }

    pub fn update_date(&mut self) {
        if let Some(backend) = &mut self.backend {
            let date_label = find_child(self.main_window, "date").unwrap() as *mut Label;

            let (day, month, year) = backend.calendar().get_date();
            unsafe {
                (*date_label).set_text(&qt_string!(format!("{:02}.{:02}.{:04}", day, month, year)));
            }
        }
    }

    pub fn add_time(&mut self, delta: &TimeUnits) {
        if let Some(backend) = &mut self.backend {
            backend.calendar().advance_time(*delta);
            self.update_time();
            self.update_date();
        }
    }

    pub fn set_date(&mut self) {
        if let Some(backend) = &mut self.backend {
            let cal = backend.calendar();
            let dialog = load("ui/date_dialog.ui") as *mut Dialog;

            let year_label = find_child(dialog, "year").unwrap() as *mut Label;
            let month_label = find_child(dialog, "month").unwrap() as *mut Label;

            let (day, month, year) = cal.get_date();

            unsafe {
                (*year_label).set_text(&qt_string!(year));
                (*month_label).set_text(&qt_string!(format!(
                    "{} ({})",
                    cal.get_month_name(month),
                    month
                )));
                (*(find_child(dialog, format!("day{}", day).as_str()).unwrap() as *mut PushButton))
                    .set_checked(true);
            }

            struct Callback {}

            impl Callback {
                pub fn prev_year(
                    &mut self,
                    (_, year, dialog, _): &mut (i32, i32, *mut Dialog, &mut PenAndPaperCalendar),
                ) {
                    *year -= 1;

                    let year_label = find_child(*dialog, "year").unwrap() as *mut Label;
                    unsafe {
                        (*year_label).set_text(&qt_string!(*year));
                    }
                }

                pub fn next_year(
                    &mut self,
                    (_, year, dialog, _): &mut (i32, i32, *mut Dialog, &mut PenAndPaperCalendar),
                ) {
                    *year += 1;

                    let year_label = find_child(*dialog, "year").unwrap() as *mut Label;
                    unsafe {
                        (*year_label).set_text(&qt_string!(*year));
                    }
                }

                pub fn prev_month(
                    &mut self,
                    (month, _, dialog, calendar): &mut (
                        i32,
                        i32,
                        *mut Dialog,
                        &mut PenAndPaperCalendar,
                    ),
                ) {
                    *month = ((*month - 2) % calendar.months_per_year()) + 1;
                    if *month < 1 {
                        *month += 12
                    }

                    let month_label = find_child(*dialog, "month").unwrap() as *mut Label;
                    unsafe {
                        (*month_label).set_text(&qt_string!(format!(
                            "{} ({})",
                            calendar.get_month_name(*month),
                            month
                        )));
                    }
                }

                pub fn next_month(
                    &mut self,
                    (month, _, dialog, calendar): &mut (
                        i32,
                        i32,
                        *mut Dialog,
                        &mut PenAndPaperCalendar,
                    ),
                ) {
                    *month = (*month % calendar.months_per_year()) + 1;

                    let month_label = find_child(*dialog, "month").unwrap() as *mut Label;
                    unsafe {
                        (*month_label).set_text(&qt_string!(format!(
                            "{} ({})",
                            calendar.get_month_name(*month),
                            month
                        )));
                    }
                }
            }

            let mut callback = Callback {};
            let mut args = (month, year, dialog, cal);

            connect!(
                find_child(dialog, "prev_year").unwrap(),
                SIGNAL!("pressed()"),
                &mut callback,
                Callback,
                Callback::prev_year,
                &mut args,
                &mut (i32, i32, *mut Dialog, &mut PenAndPaperCalendar)
            );
            connect!(
                find_child(dialog, "next_year").unwrap(),
                SIGNAL!("pressed()"),
                &mut callback,
                Callback,
                Callback::next_year,
                &mut args,
                &mut (i32, i32, *mut Dialog, &mut PenAndPaperCalendar)
            );

            connect!(
                find_child(dialog, "prev_month").unwrap(),
                SIGNAL!("pressed()"),
                &mut callback,
                Callback,
                Callback::prev_month,
                &mut args,
                &mut (i32, i32, *mut Dialog, &mut PenAndPaperCalendar)
            );
            connect!(
                find_child(dialog, "next_month").unwrap(),
                SIGNAL!("pressed()"),
                &mut callback,
                Callback,
                Callback::next_month,
                &mut args,
                &mut (i32, i32, *mut Dialog, &mut PenAndPaperCalendar)
            );

            let result = unsafe { (*dialog).exec() };

            let buttongroup = find_child(dialog, "buttonGroup").unwrap() as *mut ButtonGroup;
            let day: i32 = unsafe {
                (*((*buttongroup).checked_button() as *mut Object))
                    .object_name()
                    .to_std_string()
                    .as_str()[3..]
                    .parse()
                    .unwrap()
            };
            let month = args.0;
            let year = args.1;

            let calendar = args.3;
            match result {
                1 => {
                    calendar.set_date(day, month, year);
                    self.update_date()
                }
                0 => (),
                x => {
                    panic!("Invalid result from QDialog::exec(): {}", x);
                }
            }
        }
    }

    pub fn next_day(&mut self) {
        if let Some(backend) = &mut self.backend {
            let calendar = backend.calendar();
            let (mut hour, mut minute) = calendar.morning();
            let time = calendar.get_time();
            hour -= time.0;
            minute -= time.1;

            let mut minutes = minute + hour * calendar.minutes_per_hour();

            if minutes <= 0 {
                minutes += calendar.hours_per_day() * calendar.minutes_per_hour();
            }

            calendar.advance_time(TimeUnits::Minutes(minutes));
            self.update_time();
            self.update_date();
        }
    }

    pub fn next_evening(&mut self) {
        if let Some(backend) = &mut self.backend {
            let calendar = backend.calendar();
            let (mut hour, mut minute) = calendar.evening();
            let time = calendar.get_time();
            hour -= time.0;
            minute -= time.1;

            let mut minutes = minute + hour * calendar.minutes_per_hour();

            if minutes <= 0 {
                minutes += calendar.hours_per_day() * calendar.minutes_per_hour();
            }

            calendar.advance_time(TimeUnits::Minutes(minutes));
            self.update_time();
            self.update_date();
        }
    }
}
