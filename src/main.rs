#[macro_use]
extern crate bitflags;
extern crate libloading as lib;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_yaml;

use std::os::raw::c_char;
use std::ptr;

mod vk;
mod vulkan;

fn enumerate_extension_for_layer(functions: &vulkan::Entry, layer_name: *const c_char) -> vulkan::Result<()> {
    let extensions = functions.enumerate_instance_extension_properties(layer_name)?;
    for extension in &extensions {
        println!("{}", serde_yaml::to_string(extension).unwrap());
    }
    Ok(())
}

fn main() {
    println!("Loading library");

    let entry = vulkan::Entry::new().unwrap();

    println!("Enumerating layers");

    let layers = entry.enumerate_instance_layer_properties().unwrap();

    println!("\nFound {} layers: ", layers.len());

    enumerate_extension_for_layer(&entry, ptr::null()).unwrap();

    for layer in &layers {
        println!("{}", serde_yaml::to_string(layer).unwrap());
        enumerate_extension_for_layer(&entry, layer.layer_name.0.as_ptr()).unwrap();
    }

    let instance = entry.create_instance().unwrap();

    let physical_devices = instance.enumerate_physical_devices().unwrap();

    println!("\nFound {} devices: ", physical_devices.len());

    for physical_device in physical_devices {
        let features = instance.get_physical_device_features(physical_device);
        println!("\nFeatures\n{}", serde_yaml::to_string(&features).unwrap());
        let properties = instance.get_physical_device_properties(physical_device);
        println!("\nProperties\n{}", serde_yaml::to_string(&properties).unwrap());
        let queue_family_properties = instance.get_physical_device_queue_family_properties(physical_device);
        println!("\nQueue Family Properties\n{}", serde_yaml::to_string(&queue_family_properties).unwrap());
    }

    instance.destroy_instance();
}
