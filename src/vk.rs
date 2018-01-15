use std::os::raw::c_char;
use std::os::raw::c_void;

const MAX_EXTENSION_NAME_SIZE: usize = 256;
const MAX_DESCRIPTION_SIZE: usize = 256;

pub type Instance = *const c_void;

#[repr(i32)]
#[derive(Debug, PartialEq)]
pub enum Result {
    Success = 0,
    MaxEnum = 0x7fffffff,
}

#[repr(i32)]
#[derive(Debug, PartialEq)]
pub enum StructureType {
    ApplicationInfo = 0,
    InstanceCreateInfo = 1,
    MaxEnum = 0x7fffffff,
}

#[repr(i32)]
#[derive(Debug, PartialEq)]
pub enum InstanceCreateFlags {
    Reserved = 0,
}

#[repr(C)]
pub struct LayerProperties {
    pub layer_name: [c_char; MAX_EXTENSION_NAME_SIZE],
    pub spec_version: u32,
    pub implementation_version: u32,
    pub description: [c_char; MAX_DESCRIPTION_SIZE],
}

#[repr(C)]
pub struct ExtensionProperties {
    pub extension_name: [c_char; MAX_EXTENSION_NAME_SIZE],
    pub spec_version: u32,
}

#[repr(C)]
pub struct AllocationCallbacks {
    p_user_data: *const c_void,
    pfn_allocation: *const c_void,
    pfn_reallocation: *const c_void,
    pfn_free: *const c_void,
    pfn_internal_allocation: *const c_void,
    pfn_internal_free: *const c_void,
}

#[repr(C)]
pub struct ApplicationInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_application_name: *const c_char,
    pub application_version: u32,
    pub p_engine_name: *const c_char,
    pub engine_version: u32,
    pub api_version: u32,
}

#[repr(C)]
pub struct InstanceCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: InstanceCreateFlags,
    pub p_application_info: *const ApplicationInfo,
    pub enabled_layer_count: u32,
    pub pp_enabled_layer_names: *const *const c_char,
    pub enabled_extension_count: u32,
    pub pp_enabled_extension_names: *const *const c_char,
}