// smartctl.rs

use chrono::DateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// the output of a `smartctl --all --json` command; a `serde_json::Value`
/// wrapped up to provide a convenience method for extraction of the
/// `serial_number` field found in smartctl output
#[derive(Debug, Deserialize, Serialize)]
pub struct SmartCtl(pub Value);

impl SmartCtl {
    pub fn get_date_created(&self) -> Option<String> {
        // from the wrapped JSON Value
        self.0
            // find the `local_time` field
            .get("local_time")
            // find the `time_t` field
            .and_then(|local_time| local_time.get("time_t"))
            // convert the integer there to an i64
            .and_then(|time_t| time_t.as_i64())
            // convert that into an ISO8601 date
            .and_then(|timestamp| {
                // convert the integer into a DateTime<Utc>
                let datetime = DateTime::from_timestamp(timestamp, 0)?;
                // format the DateTime<Utc> as an ISO8601 date
                Some(datetime.to_rfc3339().replace("+00:00", "Z"))
            })
    }

    pub fn get_serial(&self) -> Option<String> {
        // from the wrapped JSON Value
        self.0
            // find the `serial_number` field
            .get("serial_number")
            // and attempt to extract it as an owned String
            .and_then(|serial_number| serial_number.as_str().map(String::from))
    }
}

//---------------------------------------------------------------------------
//---------------------------------------------------------------------------
//---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_always_succeed() {
        assert!(true);
    }

    #[test]
    fn test_get_serial() {
        // test that get_serial can extract a serial number from the JSON Value
        // hey wait a minute, those funky integers aren't valid JSON!
        // See: https://github.com/serde-rs/json/issues/974
        let smartctl_json = json!({
            "json_format_version": [
              1,
              0
            ],
            "smartctl": {
              "version": [
                7,
                1
              ],
              "svn_revision": "5080",
              "platform_info": "x86_64-linux-4.18.0-305.17.1.el8_4.x86_64",
              "build_info": "(local build)",
              "argv": [
                "smartctl",
                "--all",
                "--json",
                "/dev/slot1"
              ],
              "exit_status": 0
            },
            "device": {
              "name": "/dev/slot1",
              "info_name": "/dev/slot1 [SAT]",
              "type": "sat",
              "protocol": "ATA"
            },
            "model_name": "ST16000NM000J-2TW103",
            "serial_number": "ZRS1NWBL",
            "wwn": {
              "naa": 5,
              "oui": 3152,
              "id": 3906377770_i64
            },
            "firmware_version": "SN04",
            "user_capacity": {
              "blocks": 31251759104_i64,
              "bytes": 16000900661248_i64
            },
            "logical_block_size": 512,
            "physical_block_size": 4096
        });
        let smartctl = SmartCtl(smartctl_json);
        assert_eq!(smartctl.get_serial(), Some("ZRS1NWBL".to_string()));
    }

    #[test]
    fn test_get_serial_none() {
        // test that get_serial returns None when serial number is missing from the JSON Value
        let smartctl_no_serial = SmartCtl(json!({}));
        assert_eq!(smartctl_no_serial.get_serial(), None);
    }
}
