use lib;
use std::default::{Default};
use std::ffi::CStr;
use std::io;
use std::mem;
use std::ops::Deref;
use std::os::raw::c_char;
use std::os::raw::c_void;
use std::ptr;
use std::rc::{Rc};
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

    fn load<T>(&self, instance: vk::Instance, name: &[u8]) -> io::Result<T> {
        unsafe {
            let cname = CStr::from_bytes_with_nul_unchecked(name);
            let function = (self.get_instance_proc_addr)(instance, cname.as_ptr());
            if function == ptr::null() {
                return Err(io::Error::from(io::ErrorKind::NotFound));
            }
            Ok(mem::transmute_copy(&function))
        }
    }
}

pub struct Entry {
    #[allow(dead_code)]
    library: Rc<Library>,
    create_instance: unsafe extern fn(*const vk::InstanceCreateInfo, *const vk::AllocationCallbacks, *mut vk::Instance) -> vk::Result,
    enumerate_instance_extension_properties: unsafe extern fn(*const c_char, *mut u32, *mut vk::ExtensionProperties) -> vk::Result,
    enumerate_instance_layer_properties: unsafe extern fn(*mut u32, *mut vk::LayerProperties) -> vk::Result,
}

impl Entry {

    pub fn new() -> io::Result<Entry> {
        let library = Rc::new(Library::new()?);

        let create_instance = library.load(ptr::null(), b"vkCreateInstance\0")?;
        let enumerate_instance_extension_properties = library.load(ptr::null(), b"vkEnumerateInstanceExtensionProperties\0")?;
        let enumerate_instance_layer_properties = library.load(ptr::null(), b"vkEnumerateInstanceLayerProperties\0")?;

        Ok(Entry {
            library: library,
            create_instance: create_instance,
            enumerate_instance_extension_properties: enumerate_instance_extension_properties,
            enumerate_instance_layer_properties: enumerate_instance_layer_properties,
        })
    }

    pub fn create_instance(&self) -> io::Result<Instance> {
        unsafe {
            let create_info = vk::InstanceCreateInfo {
                s_type: vk::StructureType::InstanceCreateInfo,
                p_next: ptr::null(),
                flags: vk::InstanceCreateFlags::Reserved,
                p_application_info: ptr::null(),
                enabled_layer_count: 0,
                pp_enabled_layer_names: ptr::null(),
                enabled_extension_count: 0,
                pp_enabled_extension_names: ptr::null(),
            };
            let mut instance: vk::Instance = ptr::null();

            let err = (self.create_instance)(&create_info, ptr::null(), &mut instance);
            if err != vk::Result::Success {
                return Err(io::Error::from(io::ErrorKind::Other));
            }

            Instance::new(&self.library, instance)
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
}

pub struct Instance {
    #[allow(dead_code)]
    library: Rc<Library>,
    instance: vk::Instance,
    destroy_instance: unsafe extern fn(vk::Instance, *const vk::AllocationCallbacks),
    enumerate_physical_devices: unsafe extern fn(vk::Instance, *mut u32, *mut vk::PhysicalDevice) -> vk::Result,
    get_physical_device_features: unsafe extern fn(vk::PhysicalDevice, *mut vk::PhysicalDeviceFeatures),
    get_physical_device_properties: unsafe extern fn(vk::PhysicalDevice, *mut vk::PhysicalDeviceProperties),
    get_physical_device_queue_family_properties: unsafe extern fn(vk::PhysicalDevice, *mut u32, *mut vk::QueueFamilyProperties),
}

impl Instance {
    fn new(library: &Rc<Library>, instance: vk::Instance) -> io::Result<Instance> {
        let destroy_instance = library.load(instance, b"vkDestroyInstance\0")?;
        let enumerate_physical_devices = library.load(instance, b"vkEnumeratePhysicalDevices\0")?;
        let get_physical_device_features = library.load(instance, b"vkGetPhysicalDeviceFeatures\0")?;
        let get_physical_device_properties = library.load(instance, b"vkGetPhysicalDeviceProperties\0")?;
        let get_physical_device_queue_family_properties = library.load(instance, b"vkGetPhysicalDeviceQueueFamilyProperties")?;

        Ok(Instance {
            library: library.clone(),
            instance: instance,
            destroy_instance: destroy_instance,
            enumerate_physical_devices: enumerate_physical_devices,
            get_physical_device_features: get_physical_device_features,
            get_physical_device_properties: get_physical_device_properties,
            get_physical_device_queue_family_properties: get_physical_device_queue_family_properties,
        })
    }

    pub fn destroy_instance(&self) {
        unsafe {
            (self.destroy_instance)(self.instance, ptr::null());
        }
    }

    pub fn enumerate_physical_devices(&self) -> Result<Vec<vk::PhysicalDevice>> {
        unsafe {
            let mut count: u32 = 0;

            let err = (self.enumerate_physical_devices)(self.instance, &mut count, ptr::null_mut());
            if err != vk::Result::Success {
                return Err(err);
            }

            let mut devices = Vec::with_capacity(count as usize);
            devices.set_len(count as usize);

            let err = (self.enumerate_physical_devices)(self.instance, &mut count, devices.as_mut_ptr());
            if err != vk::Result::Success {
                return Err(err);
            }

            Ok(devices)
        }
    }

    pub fn get_physical_device_features(&self, physical_device: vk::PhysicalDevice) -> vk::PhysicalDeviceFeatures {
        unsafe {
            let mut features: vk::PhysicalDeviceFeatures;
            features = mem::uninitialized();
            (self.get_physical_device_features)(physical_device, &mut features);
            features
        }
    }

    pub fn get_physical_device_properties(&self, physical_device: vk::PhysicalDevice) -> vk::PhysicalDeviceProperties {
        unsafe {
            let mut properties: vk::PhysicalDeviceProperties;
            properties = mem::uninitialized();
            (self.get_physical_device_properties)(physical_device, &mut properties);
            properties
        }
    }

    pub fn get_physical_device_queue_family_properties(&self, physical_device: vk::PhysicalDevice) -> Vec<vk::QueueFamilyProperties> {
        unsafe {
            let mut count: u32 = 0;
            (self.get_physical_device_queue_family_properties)(physical_device, &mut count, ptr::null_mut());
            let mut properties = Vec::with_capacity(count as usize);
            properties.set_len(count as usize);
            (self.get_physical_device_queue_family_properties)(physical_device, &mut count, properties.as_mut_ptr());
            properties
        }
    }
}
