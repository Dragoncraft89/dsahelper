#![macro_use]
use std::os::raw::{c_char, c_void};

use std::mem::transmute;

use std::sync::Mutex;

use std::collections::HashMap;

use qt_widgets::cpp_utils::StaticCast;

use qt_widgets::qt_core;

use qt_core::object::Object;
use qt_core::string::String;

use qt_widgets::widget::Widget;

use qt_widgets::layout::Layout;

use qt_widgets::cpp_utils::CppBox;

use qt_core::file::File;
use qt_core::flags::Flags;
use qt_core::io_device::{IODevice, OpenModeFlag};

use qt_ui_tools::ui_loader::UiLoader;

use qt_core::list::ListModelIndex;
use qt_core::model_index::ModelIndex;

use qt_widgets::input_dialog::InputDialog;
use qt_widgets::line_edit::EchoMode;

extern "C" {
    fn create(
        obj: *mut c_void,
        signal: *const c_char,
        arg: *mut c_void,
        callback: *const c_void,
        argument: *const c_void,
    ) -> *mut c_void;
    fn destroy(obj: *mut c_void);
}

#[no_mangle]
pub extern "C" fn rust_free(obj: *mut c_void) {
    unsafe { Box::from_raw(obj) };
}

pub fn as_object<T: StaticCast<Object>>(w: *mut T) -> *mut Object {
    unsafe {
        w.as_mut()
            .map(|obj| T::static_cast_mut(obj) as *mut Object)
            .unwrap_or(std::ptr::null_mut())
    }
}

pub struct BindManager {
    connections: HashMap<(*mut Object, &'static [u8]), *mut c_void>,
}

impl BindManager {
    pub fn new() -> BindManager {
        BindManager {
            connections: HashMap::new(),
        }
    }

    pub fn connect(
        &mut self,
        obj: *mut Object,
        s: &'static [u8],
        arg: *mut c_void,
        callback: *mut c_void,
        argument: *mut c_void,
    ) {
        if let Some(var) = self.connections.insert((obj, s), unsafe {
            create(
                transmute(obj),
                s.as_ptr() as *const c_char,
                transmute(arg),
                callback,
                argument,
            )
        }) {
            unsafe { destroy(var) }
        };
    }

    /*
    pub fn disconnect(&mut self, obj: *mut Object, s: &'static [u8]) {
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

unsafe impl Send for BindManager {}

lazy_static! {
    pub static ref MANAGER: Mutex<BindManager> = Mutex::new(BindManager::new());
}

#[macro_export]
macro_rules! connect {
    ($sender:expr, $signal:expr, $data:expr, $call_type:ident, $callback:path) => {{
        extern "C" fn temp_call(
            target: *mut std::os::raw::c_void,
            _arg: *mut std::os::raw::c_void,
        ) {
            unsafe {
                let app = target as *mut $call_type;
                $callback(&mut *app);
            }
        }

        let data: &mut $call_type = $data;
        #[allow(unused_unsafe)]
        unsafe {
            qt_bind::MANAGER.lock().unwrap().connect(
                $sender as *mut qt_widgets::qt_core::object::Object,
                $signal,
                std::mem::transmute(data),
                temp_call as *mut std::os::raw::c_void,
                0 as *mut std::os::raw::c_void,
            )
        }
    }};
    ($sender:expr, $signal:expr, $data:expr, $call_type:ident, $callback:path, $argument:expr, $argtype:ty) => {{
        extern "C" fn temp_call(target: *mut std::os::raw::c_void, arg: *mut std::os::raw::c_void) {
            let app = target as *mut $call_type;
            unsafe {
                let argument = Box::leak(Box::from_raw(arg as *mut $argtype));
                $callback(&mut *app, argument);
            }
        }

        // type checking of arguments
        // if you mess the types up, you're gonna have a lot of pain
        let arg: $argtype = $argument;
        let data: &mut $call_type = $data;
        #[allow(unused_unsafe)]
        unsafe {
            let ptr =
                std::boxed::Box::into_raw(std::boxed::Box::new(arg)) as *mut std::os::raw::c_void;
            qt_bind::MANAGER.lock().unwrap().connect(
                $sender as *mut qt_widgets::qt_core::object::Object,
                $signal,
                std::mem::transmute(data),
                temp_call as *mut std::os::raw::c_void,
                ptr,
            )
        }
    }};
}

#[macro_export]
macro_rules! disconnect {
    ($sender:expr, $signal:expr, $data:expr, $call_type:ident, $callback:path) => {{
        qt_bind::MANAGER
            .lock()
            .unwrap()
            .disconnect($sender, $signal)
    }};
}

pub fn find_child<R, T: StaticCast<Object>>(t: *mut T, name: &str) -> Option<*mut R> {
    let obj = as_object(t);
    find_child_internal(obj, name)
}

fn find_child_internal<R>(obj: *mut Object, name: &str) -> Option<*mut R> {
    if unsafe { (*obj).object_name().compare(&String::from(name)) == 0 } {
        return Some(obj as *mut R);
    }

    let children = unsafe { (*obj).children() };
    for i in 0..children.size() {
        if let Some(x) = find_child_internal(*children.at(i), name) {
            return Some(x);
        }
    }
    None
}

pub fn find_child_layout<R, T: StaticCast<Layout>>(t: *mut T, name: &str) -> Option<*mut R> {
    let layout = unsafe {
        let obj: &mut T = Box::leak(Box::from_raw(t));

        T::static_cast_mut(obj) as *mut Layout
    };

    find_child_layout_internal(layout, name)
}

fn find_child_layout_internal<R>(layout: *mut Layout, name: &str) -> Option<*mut R> {
    unsafe {
        for i in 0..(*layout).count() {
            let child = (*layout).item_at(i);

            let widget = (*child).widget();

            if widget.is_null() {
                continue;
            }

            if let Some(x) = find_child_internal(as_object(widget), name) {
                return Some(x);
            }
        }
    }

    None
}

#[macro_export]
macro_rules! SIGNAL {
    ($s: expr) => {
        concat!("2", concat!($s, "\0")).as_bytes()
    };
}

pub fn delete(obj: *mut Object) {
    unsafe {
        CppBox::new(obj);
    }
}

pub fn load(s: &str) -> *mut Widget {
    let mut f = File::new(&String::from(s));
    f.open(Flags::from_enum(OpenModeFlag::ReadOnly));
    let mut loader = UiLoader::new();
    let obj = unsafe { loader.load(File::static_cast_mut(&mut f) as *mut IODevice) };

    if obj.is_null() {
        panic!("Failed to load: {}", s);
    }

    obj
}

pub trait StringConversionHelper {
    fn process(self) -> String;
}

impl StringConversionHelper for &std::string::String {
    fn process(self) -> String {
        String::from(self.as_str())
    }
}

impl StringConversionHelper for std::string::String {
    fn process(self) -> String {
        String::from(self.as_str())
    }
}

impl StringConversionHelper for &str {
    fn process(self) -> String {
        String::from(self)
    }
}

impl StringConversionHelper for i32 {
    fn process(self) -> String {
        String::number0(self)
    }
}

#[macro_export]
macro_rules! qt_string {
    ($s: expr) => {
        crate::qt_bind::StringConversionHelper::process($s)
    };
}

pub struct ListModelIterator<'a> {
    index: i32,
    list: &'a ListModelIndex,
}

impl<'a> Iterator for ListModelIterator<'a> {
    type Item = &'a ModelIndex;

    fn next(&mut self) -> Option<&'a ModelIndex> {
        self.index += 1;

        if self.index <= self.list.size() {
            return Some(self.list.at(self.index - 1));
        }

        None
    }
}

pub fn iter<'a>(list: &'a ListModelIndex) -> ListModelIterator<'a> {
    ListModelIterator {
        index: 0,
        list: list,
    }
}

pub fn input(
    window: *mut Widget,
    title: &str,
    label: &str,
    text: &str,
) -> Option<std::string::String> {
    let mut ok = false;
    let name = unsafe {
        InputDialog::get_text((
            window,
            &qt_string!(title),
            &qt_string!(label),
            EchoMode::Normal,
            &qt_string!(text),
            &mut ok as *mut bool,
        ))
    };

    match ok {
        true => Some(name.to_std_string()),
        false => None,
    }
}
