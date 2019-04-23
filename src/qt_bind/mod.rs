use std::os::raw::{c_void, c_char};

use std::mem::transmute;

use std::sync::Mutex;

use std::collections::HashMap;

use qt_widgets::cpp_utils::StaticCast;

use qt_widgets::qt_core;

use qt_core::object::Object;
use qt_core::string::String;

extern "C" {
    fn create(obj: *mut c_void, signal: *const c_char, arg: *mut c_void, callback: *const c_void) -> *mut c_void;
    fn destroy(obj: *mut c_void);
}

fn as_object<T: StaticCast<Object>>(w: *mut T) -> *mut Object {
    unsafe {
        let obj: &mut T = Box::leak(Box::from_raw(w));
        
        T::static_cast_mut(obj) as *mut Object
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

unsafe impl Send for BindManager {}

lazy_static! {
    pub static ref MANAGER: Mutex<BindManager> = Mutex::new(BindManager::new());
}

#[macro_export]
macro_rules! connect {
    ($sender:expr, $signal:expr, $data:expr, $call_type:ident, $callback:path) => {
        {
            extern "C" fn temp_call(target: *mut std::os::raw::c_void) {
                unsafe {
                    let app = target as *mut $call_type; 
                    $callback(&mut *app);
                }
            }

            unsafe {
                qt_bind::MANAGER.lock().unwrap().connect($sender, $signal, std::mem::transmute($data), temp_call as *mut std::os::raw::c_void)
            }
        }
    }
}

pub fn find_child<T: StaticCast<Object>>(t: *mut T, name: &str) -> Option<*mut Object> {
    let obj = as_object(t);
    find_child_internal(obj, name)
}

fn find_child_internal(obj: *mut Object, name: &str) -> Option<*mut Object> {
    let children = unsafe {(*obj).children()};
    for i in 0..children.size() {
        if unsafe {(**children.at(i)).object_name().compare(&String::from(name)) == 0} {
            return Some(*children.at(i));
        }
        
        match find_child_internal(*children.at(i), name) {
            Some(x) => Some(x),
            None => continue
        };
    }
    None
}

#[macro_export]
macro_rules! SIGNAL {
    ($s: expr) => {
        concat!("2", concat!($s, "\0")).as_bytes()
    }
}
