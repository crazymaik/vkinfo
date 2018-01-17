extern crate libloading as lib;

use std::os::raw::c_char;
use std::ptr;

mod vk;
mod vulkan;

fn enumerate_extension_for_layer(functions: &vulkan::Entry, layer_name: *const c_char) -> vulkan::Result<()> {
    let extensions = functions.enumerate_instance_extension_properties(layer_name)?;
    for extension in &extensions {
        println!("{:?}", extension);
    }
    Ok(())
}

fn main() {
    println!("Loading library");

    let entry = vulkan::Entry::new().unwrap();

    println!("Enumerating layers");

    let layers = entry.enumerate_instance_layer_properties().unwrap();

    println!("Found {} layers: ", layers.len());

    enumerate_extension_for_layer(&entry, ptr::null()).unwrap();

    for layer in &layers {
        println!("{:?}", layer);
        enumerate_extension_for_layer(&entry, layer.layer_name.0.as_ptr()).unwrap();
    }

    let instance = entry.create_instance().unwrap();

    let physical_devices = instance.enumerate_physical_devices().unwrap();

    println!("Found {} devices: ", physical_devices.len());

    for physical_device in physical_devices {
        let features = instance.get_physical_device_features(physical_device);
        println!("Features\n{:?}", features);
        let properties = instance.get_physical_device_properties(physical_device);
        println!("Properties\n{:?}", properties);
        let queue_family_properties = instance.get_physical_device_queue_family_properties(physical_device);
        println!("Queue Family Properties\n{:?}", queue_family_properties);
    }

    instance.destroy_instance();
}
