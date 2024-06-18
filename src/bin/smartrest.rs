// smartrest.rs

use gotham::{
    router::{build_simple_router, response::StaticResponseExtender, Router},
    state::StateData,
};
use gotham_restful::{read_all, search, DrawResources, Resource, Success};
use log::{error, info};
use serde::Deserialize;
use serde_json::{json, Value};
use std::process::Command;

// --------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------

#[derive(Resource)]
#[resource(smartctl_scan)]
#[resource(smartctl_all)]
struct SmartCtlResource;

#[derive(Clone, Deserialize, StateData, StaticResponseExtender)]
struct DevicePath {
    device: String,
}

#[read_all]
fn smartctl_scan() -> Success<Value> {
    execute_smartctl_scan().into()
}

#[search]
fn smartctl_all(path: DevicePath) -> Success<Value> {
    execute_smartctl_all(&path.device).into()
}

// --------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------

fn execute_smartctl_all(device: &str) -> Value {
    // enumerate all the devices on the system
    let mut smartctl_devices = Vec::new();

    // query smartctl to ask what devices we have on the system
    let smartctl = execute_smartctl_scan();
    // if we got back some useful data from smartctl
    if let Some(smartctl_obj) = smartctl.as_object() {
        // if there is a 'devices' array
        if let Some(devices) = smartctl_obj.get("devices") {
            if let Some(devices_array) = devices.as_array() {
                // for each device in the array
                for smartctl_device in devices_array {
                    // if the device has a name
                    if let Some(device_name) = smartctl_device.get("name") {
                        if let Some(name) = device_name.as_str() {
                            // add it to the list of devices
                            smartctl_devices.push(name);
                        }
                    }
                }
            }
        }
    }

    // check to see if the provided device is on the list
    if smartctl_devices.contains(&device) {
        // if we get output from the smartctl command
        if let Ok(output) = Command::new("/usr/sbin/smartctl")
            .arg("--all") // give us all the information for the device
            .arg("--json") // give us the result in JSON
            .arg(device) // about this device
            .output()
        {
            // if we can convert that output into a sensible utf8 string
            if let Ok(stdout) = String::from_utf8(output.stdout) {
                // if we can parse that result into a JSON value
                if let Ok(result) = serde_json::from_str(&stdout) {
                    // give the output to the caller
                    return result;
                }
            }
        }
    }

    // give the caller an empty object
    error!(
        "Device {} was not found in the list of smartctl devices: {:?}",
        device, smartctl_devices
    );
    json!("{}")
}

fn execute_smartctl_scan() -> Value {
    // if we get output from the smartctl command
    if let Ok(output) = Command::new("/usr/sbin/smartctl")
        .arg("--scan") // scan for devices
        .arg("--json") // give us the result in JSON
        .output()
    {
        // if we can convert that output into a sensible utf8 string
        if let Ok(stdout) = String::from_utf8(output.stdout) {
            // if we can parse that result into a JSON value
            if let Ok(result) = serde_json::from_str(&stdout) {
                // give the output to the caller
                return result;
            }
        }
    }

    // give the caller an empty object
    json!("{}")
}

// --------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------

fn router() -> Router {
    build_simple_router(|route| {
        route.resource::<SmartCtlResource>("smartctl");
    })
}

pub fn main() {
    // initialize logging, configured by environment
    env_logger::init();
    // start the service
    let addr = "0.0.0.0:8080";
    info!("Listening for requests at http://{}", addr);
    let _ = gotham::start(addr, router());
}
