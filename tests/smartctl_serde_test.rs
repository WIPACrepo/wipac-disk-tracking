// smartctl_serde_test.rs

use serde_json::Value;
use std::fs::read_to_string;
use wipac_disk_tracking::smartctl::SmartCtl;

#[test]
fn test_deserialize_smartctl() {
    // read JSON data
    let json_str = read_to_string("tests/data/smartctl001.json").expect("Unable to read file");
    // deserialize JSON data
    let smartctl_json: Value = serde_json::from_str(&json_str).expect("Unable to deserialize JSON");
    // wrap in a SmartCtl struct
    let smartctl = SmartCtl(smartctl_json);
    // obtain the serial number of the disk
    let serial = smartctl.get_serial();
    // ensure we got the correct serial number from the test data
    assert_eq!(serial, Some("ZRS1NWBL".to_string()));
}
