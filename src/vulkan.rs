use lib;
use std::ffi::CStr;
use std::io;
use std::mem;
use std::ops::Deref;
use std::os::raw::c_char;
use std::os::raw::c_void;
use std::ptr;
use std::result;

use vk;

pub type Result<T> = result::Result<T, vk::Result>;

struct Library {
    #[allow(dead_code)]
    library: lib::Library,
    get_instance_proc_addr: unsafe extern fn(*const c_void, *const c_char) -> *const c_void,
}

impl Library {
    fn new() -> io::Result<Library> {
        let library = lib::Library::new("vulkan-1.dll")?;

        let get_instance_proc_addr = unsafe {
            let f: lib::Symbol<unsafe extern fn(*const c_void, *const c_char) -> *const c_void> = library.get(b"vkGetInstanceProcAddr\0")?;
            *f.into_raw().deref()
        };

        Ok(Library {
            library: library,
            get_instance_proc_addr: get_instance_proc_addr,
        })
    }

    fn load<T>(&self, name: &[u8]) -> io::Result<T> {
        unsafe {
            let cname = CStr::from_bytes_with_nul_unchecked(name);
            let function = (self.get_instance_proc_addr)(ptr::null(), cname.as_ptr());
            if function == ptr::null() {
                return Err(io::Error::from(io::ErrorKind::NotFound));
            }
            Ok(mem::transmute_copy(&function))
        }
    }
}

pub struct Functions {
    #[allow(dead_code)]
    library: Library,
    enumerate_instance_layer_properties: unsafe extern fn(*mut u32, *mut vk::LayerProperties) -> vk::Result,
    enumerate_instance_extension_properties: unsafe extern fn(*const c_char, *mut u32, *mut vk::ExtensionProperties) -> vk::Result,
}

impl Functions {

    pub fn new() -> io::Result<Functions> {
        let library = Library::new()?;

        let enumerate_instance_layer_properties = library.load(b"vkEnumerateInstanceLayerProperties\0")?;
        let enumerate_instance_extension_properties = library.load(b"vkEnumerateInstanceExtensionProperties\0")?;

        Ok(Functions {
            library: library,
            enumerate_instance_layer_properties: enumerate_instance_layer_properties,
            enumerate_instance_extension_properties: enumerate_instance_extension_properties,
        })
    }

    pub fn enumerate_instance_layer_properties(&self) -> Result<Vec<vk::LayerProperties>> {
        unsafe {
            let mut count: u32 = 0;

            let err = (self.enumerate_instance_layer_properties)(&mut count, ptr::null_mut());
            if err != vk::Result::Success {
                return Err(err)
            }

            let mut layers = Vec::with_capacity(count as usize);
            layers.set_len(count as usize);

            let err = (self.enumerate_instance_layer_properties)(&mut count, layers.as_mut_ptr());
            if err != vk::Result::Success {
                return Err(err)
            }

            Ok(layers)
        }
    }

    pub fn enumerate_instance_extension_properties(&self, layer_name: *const c_char) -> Result<Vec<vk::ExtensionProperties>> {
        unsafe {
            let mut count: u32 = 0;

            let err = (self.enumerate_instance_extension_properties)(layer_name, &mut count, ptr::null_mut());
            if err != vk::Result::Success {
                return Err(err)
            }

            let mut extensions = Vec::with_capacity(count as usize);
            extensions.set_len(count as usize);

            let err = (self.enumerate_instance_extension_properties)(layer_name, &mut count, extensions.as_mut_ptr());
            if err != vk::Result::Success {
                return Err(err)
            }

            Ok(extensions)
        }
    }
}