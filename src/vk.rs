use std::os::raw::c_char;

const MAX_EXTENSION_NAME_SIZE: usize = 256;
const MAX_DESCRIPTION_SIZE: usize = 256;

#[repr(i32)]
#[derive(Debug, PartialEq)]
pub enum Result {
    Success = 0,
    MaxEnum = 0x7fffffff,
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
