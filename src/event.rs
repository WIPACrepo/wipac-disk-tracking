// event.rs

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::smartctl::SmartCtl;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Event {
    /// this disk was determined to full/finished and archival activity stopped
    CLOSED,
    /// this disk was given a file system to make it ready for archival purposes
    FORMATTED,
    /// this disk was given a label and was designated for active archival activity
    OPENED,
    /// this disk was observed to be loaded in a host that processes archival disks
    SIGHTED,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DiskEvent {
    /// unique identifier for this DiskEvent
    #[serde(serialize_with = "uuid_to_string", deserialize_with = "string_to_uuid")]
    pub uuid: Uuid,
    /// iso-8601 timestamp when this DiskEvent was created
    pub date_created: String,
    /// hardware serial number of the disk this DiskEvent is about
    pub serial_number: String,
    /// the type of event this DiskEvent describes
    pub event: Event,
    /// the output from the `smartctl` command for this DiskEvent
    pub smartctl: SmartCtl,
}

fn uuid_to_string<S>(uuid: &Uuid, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&uuid.to_string())
}

fn string_to_uuid<'de, D>(deserializer: D) -> Result<Uuid, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Uuid::parse_str(&s).map_err(serde::de::Error::custom)
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
    fn test_serialization() {
        // create a uuid
        let test_uuid = Uuid::new_v4();

        // create a disk event
        let disk_event = DiskEvent {
            uuid: test_uuid,
            date_created: "2024-08-14T17:36:31Z".to_string(),
            serial_number: "ZRS1NWBL".to_string(),
            event: Event::FORMATTED,
            smartctl: SmartCtl(json!({
                "serial_number": "ZRS1NWBL"
            })),
        };

        // serialize it to JSON
        let serialized = serde_json::to_string(&disk_event).unwrap();

        // expected JSON representation
        let expected_json = json!({
            "uuid": test_uuid.to_string(),
            "date_created": "2024-08-14T17:36:31Z",
            "serial_number": "ZRS1NWBL",
            "event": "formatted",
            "smartctl": {
                "serial_number": "ZRS1NWBL"
            }
        })
        .to_string();

        // assert that the serialized JSON matches the expected JSON
        assert_eq!(serialized, expected_json);
    }
}
