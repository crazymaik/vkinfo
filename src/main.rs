extern crate libloading as lib;

use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;

mod vk;
mod vulkan;

fn enumerate_extension_for_layer(functions: &vulkan::Entry, layer_name: *const c_char) -> vulkan::Result<()> {
    let extensions = functions.enumerate_instance_extension_properties(layer_name)?;
    for extension in &extensions {
        unsafe {
            println!("  Extension");
            println!("    Name: {0}", CStr::from_ptr(extension.extension_name.as_ptr()).to_string_lossy());
            println!("    SpecVersion: {0}", extension.spec_version);
        }
    }
    Ok(())
}

fn main() {
    println!("Loading library");

    let entry = vulkan::Entry::new().unwrap();

    println!("Enumerating layers");

    let layers = entry.enumerate_instance_layer_properties().unwrap();

    println!("Found {0} layers: ", layers.len());

    enumerate_extension_for_layer(&entry, ptr::null()).unwrap();

    for layer in &layers {
        unsafe { 
            println!("Layer");
            println!("  Name: {0}", CStr::from_ptr(layer.layer_name.as_ptr()).to_string_lossy());
            println!("  SpecVersion: {0}", layer.spec_version);
            println!("  ImplementationVersion: {0}", layer.implementation_version);
            println!("  Description: {0}", CStr::from_ptr(layer.description.as_ptr()).to_string_lossy());
            enumerate_extension_for_layer(&entry, layer.layer_name.as_ptr()).unwrap();
        }
    }

    let instance = entry.create_instance().unwrap();

    instance.destroy_instance();
}
