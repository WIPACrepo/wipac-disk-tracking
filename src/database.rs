// database.rs

use futures_util::stream::TryStreamExt;
use mongodb::{
    bson::{doc, document::Document},
    results::InsertOneResult,
};
use uuid::Uuid;

use crate::context::ApplicationContext;
use crate::error::Result;
use crate::event::DiskEvent;

pub const DISK_EVENTS_COLLECTION: &str = "disk_events";

pub async fn count_disk_events(context: &ApplicationContext) -> Result<u64> {
    let client = &context.mongo_client;
    let database = client.database(&context.mongo_database);
    let collection = database.collection::<DiskEvent>(DISK_EVENTS_COLLECTION);

    let count = collection.count_documents(Document::new()).await?;

    Ok(count)
}

pub async fn find_disk_events_uuid(
    context: &ApplicationContext,
    uuid: Uuid,
) -> Result<Vec<DiskEvent>> {
    let client = &context.mongo_client;
    let database = client.database(&context.mongo_database);
    let collection = database.collection::<DiskEvent>(DISK_EVENTS_COLLECTION);
    let filter = doc! { "uuid": uuid.to_string() };

    let mut cursor = collection.find(filter).await?;
    let mut result = Vec::new();
    while let Some(doc) = cursor.try_next().await? {
        result.push(doc);
    }

    Ok(result)
}

pub async fn find_disk_events_serial_number(
    context: &ApplicationContext,
    serial_number: String,
) -> Result<Vec<DiskEvent>> {
    let client = &context.mongo_client;
    let database = client.database(&context.mongo_database);
    let collection = database.collection::<DiskEvent>(DISK_EVENTS_COLLECTION);
    let filter = doc! { "serial_number": serial_number };

    let mut cursor = collection.find(filter).await?;
    let mut result = Vec::new();
    while let Some(doc) = cursor.try_next().await? {
        result.push(doc);
    }

    Ok(result)
}

pub async fn save_disk_event(
    context: &ApplicationContext,
    disk_event: DiskEvent,
) -> Result<InsertOneResult> {
    let client = &context.mongo_client;
    let database = client.database(&context.mongo_database);
    let collection = database.collection::<DiskEvent>(DISK_EVENTS_COLLECTION);

    let result = collection.insert_one(disk_event).await?;

    Ok(result)
}

//---------------------------------------------------------------------------
//---------------------------------------------------------------------------
//---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_always_succeed() {
        assert!(true);
    }
}
