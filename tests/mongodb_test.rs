// mongodb_test.rs

use mongodb::{bson::doc, options::ClientOptions, Client, Collection};
use serde_json::Value;
use std::fs::read_to_string;
use std::time::Duration;
use uuid::Uuid;
use wipac_disk_tracking::database::DISK_EVENTS_COLLECTION;
use wipac_disk_tracking::event::{DiskEvent, Event};
use wipac_disk_tracking::smartctl::SmartCtl;

async fn get_mongodb_collection() -> Option<Collection<DiskEvent>> {
    // determine which database to use for integration testing
    // this will be a destructive test, so hopefully you didn't provide credentials to production!
    let mongo_url = std::env::var("DESTRUCTIVE_TEST_MONGODB_URL").unwrap_or_else(|_| {
        "mongodb://disk_tracking:hunter2@localhost:27017/disk_tracking".to_string()
    });

    // create a client options object and attempt to connect to MongoDB
    let mut client_options = ClientOptions::parse(&mongo_url).await.ok()?;
    client_options.connect_timeout = Some(Duration::from_secs(1));
    client_options.server_selection_timeout = Some(Duration::from_secs(1));
    let client = Client::with_options(client_options).ok()?;

    // check connection by pinging the database
    let database = client.database("admin");
    let ping_result = database.run_command(doc! {"ping": 1}).await.map(|_| ());
    if ping_result.is_err() {
        // if we couldn't ping the DB, we don't really have a connection
        return None;
    }

    // return the disk events collection to the caller
    let database = client.database("disk_tracking");
    let collection = database.collection::<DiskEvent>(DISK_EVENTS_COLLECTION);
    Some(collection)
}

#[tokio::test]
async fn test_mongodb_crud_operations() {
    // check if the disk_events collection is available
    let collection = match get_mongodb_collection().await {
        Some(coll) => coll,
        None => {
            dbg!("MongoDB is not available. Skipping test.");
            return;
        }
    };

    // create a random UUID
    let test_uuid = Uuid::new_v4();

    // create a SmartCtl from the test data
    let json_str = read_to_string("tests/data/smartctl001.json").expect("Unable to read file");
    let smartctl_json: Value = serde_json::from_str(&json_str).expect("Unable to deserialize JSON");
    let test_smartctl = SmartCtl(smartctl_json);

    // create a disk event
    let new_event = DiskEvent {
        uuid: test_uuid,
        date_created: "2024-08-14T17:36:31Z".to_string(),
        serial_number: "ZRS1NWBL".to_string(),
        event: Event::FORMATTED,
        smartctl: test_smartctl,
    };

    // insert the disk event
    let insert_result = collection.insert_one(&new_event).await;
    assert!(insert_result.is_ok());

    // read the disk event
    let filter = doc! { "serial_number": "ZRS1NWBL" };
    let found_event = collection
        .find_one(filter.clone())
        .await
        .expect("Error finding document");
    assert!(found_event.is_some());
    assert_eq!(found_event.unwrap().serial_number, "ZRS1NWBL");

    // update the disk event
    let update = doc! { "$set": { "event": "closed" } }; // we would never actually do this, but what the heck
    let update_result = collection.update_one(filter.clone(), update).await;
    assert!(update_result.is_ok());
    assert_eq!(update_result.unwrap().modified_count, 1);

    // delete the disk event
    let delete_result = collection.delete_one(filter.clone()).await;
    assert!(delete_result.is_ok());
    assert_eq!(delete_result.unwrap().deleted_count, 1);
}
