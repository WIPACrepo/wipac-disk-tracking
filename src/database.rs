// database.rs

use mongodb::{bson::document::Document, Client, options::ClientOptions};

use crate::context::ApplicationContext;
use crate::error::Result;

const DISK_EVENT_COLLECTION: &str = "disk_event";

struct DiskEvent;

pub async fn setup_mongo(context: &ApplicationContext) -> Result<Client> {
    let conn_str = context.get_mongo_url();
    let client_options = ClientOptions::parse(conn_str).await?;
    let client = Client::with_options(client_options)?;
    Ok(client)
}

pub async fn count_disk_events(context: &ApplicationContext, client: &Client) -> Result<u64> {
    let database = client.database(&context.mongo_database);
    let collection = database.collection::<DiskEvent>(DISK_EVENT_COLLECTION);
    let count  = collection.count_documents(Document::new()).await?;
    Ok(count)

//     match collection.count_documents(None, None).await {
//         Ok(count) => {
//             let body = json!({
//                 "status": "ok",
//                 "count": count,
//             });
//             (StatusCode::OK, body.to_string())
//         },
//         Err(_) => {
//             let body = json!({
//                 "status": "error",
//                 "message": "Failed to connect to the database",
//             });
//             (StatusCode::INTERNAL_SERVER_ERROR, body.to_string())
//         },
//     }

}

