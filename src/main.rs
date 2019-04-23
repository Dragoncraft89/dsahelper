extern crate qt_widgets;
mod application;

use std::os::raw::{c_void, c_char};

use std::mem::transmute;

use std::collections::HashMap;

use qt_widgets::cpp_utils::StaticCast;
use qt_widgets::qt_core;

use qt_core::object::Object;
use qt_core::string::String;
use qt_core::file::File;
use qt_core::flags::Flags;
use qt_core::io_device::{OpenModeFlag, IODevice};

use qt_ui_tools::ui_loader::UiLoader;

use application::Application;

extern "C" {
    pub fn create(obj: *mut c_void, signal: *const c_char, arg: *mut c_void, callback: *const c_void) -> *mut c_void;
    pub fn destroy(obj: *mut c_void);
}

#[macro_export]
macro_rules! connect_no_args {
    ($manager: expr, $sender:expr, $signal:expr, $data:expr, $call_type:ident, $callback:path) => {
        {
            extern "C" fn temp_call(target: *mut c_void) {
                unsafe {
                    let app = target as *mut $call_type; 
                    $callback(&mut *app);
                }
            }

            unsafe {
                $manager.connect($sender, $signal, transmute($data), temp_call as *mut c_void)
            }
        }
    }
}

pub struct BindManager {
    connections: HashMap<(*mut Object, &'static [u8]), *mut c_void>
}

impl BindManager {
    pub fn new() -> BindManager {
        BindManager { connections: HashMap::new() }
    }

    pub fn connect(&mut self, obj: *mut Object, s: &'static [u8], arg: *mut c_void , callback: *mut c_void) {
        self.connections.insert((obj, s), unsafe {create(transmute(obj), s.as_ptr() as *const c_char, transmute(arg), callback)});
    }

    /*
    fn disconnect(&mut self, obj: *mut Object, s: &'static str) {
        match self.connections.get(&(obj, s)) {
            Some(x) => unsafe { destroy(*x) }
            None => println!("Warning: Attempt to disconnect unbound method")
        };
    }
     */
}

impl Drop for BindManager {
    fn drop(&mut self) {
        for ((_obj, _signal), ptr) in &self.connections {
            unsafe { destroy(*ptr) }
        }
        
        self.connections.clear();
    }
}

fn find_child(widget: *mut Object, name: &str) -> Option<*mut Object> {
    let children = unsafe {(*widget).children()};
    for i in 0..children.size() {
        if unsafe {(**children.at(i)).object_name().compare(&String::from(name)) == 0} {
            return Some(*children.at(i));
        }
        
        match find_child(*children.at(i), name) {
            Some(x) => Some(x),
            None => continue
        };
    }
    None
}

fn as_object<T: StaticCast<Object>>(w: *mut T) -> *mut Object {
    unsafe {
        let obj: &mut T = Box::leak(Box::from_raw(w));
        
        T::static_cast_mut(obj) as *mut Object
    }
}

fn main() {
    qt_widgets::application::Application::create_and_exit(|_app| {
        let s = String::from("ui/main_window.ui");
        let mut file = File::new(&s);
        file.open(Flags::from_enum(OpenModeFlag::ReadOnly));
        let mut loader = UiLoader::new();
        let main_window;
        unsafe {
            main_window = loader.load(File::static_cast_mut(&mut file) as *mut IODevice);
            (*main_window).show();
        }

        let mut backend = Application::new();
        let mut bind_manager = BindManager::new();
        
        connect_no_args!(&mut bind_manager, find_child(as_object(main_window), "new_file").unwrap(), b"2triggered()\0", &mut backend, Application, Application::new_file);
        connect_no_args!(&mut bind_manager, find_child(as_object(main_window), "open").unwrap(), b"2triggered()\0", &mut backend, Application, Application::open);
        connect_no_args!(&mut bind_manager, find_child(as_object(main_window), "close").unwrap(), b"2triggered()\0", &mut backend, Application, Application::close);
        connect_no_args!(&mut bind_manager, find_child(as_object(main_window), "options").unwrap(), b"2triggered()\0", &mut backend, Application, Application::options);
        qt_widgets::application::Application::exec()
    })
}
