// main.rs

pub mod context;
pub mod database;
pub mod error;

// use axum::{
//     routing::get,
//     Router,
//     response::IntoResponse,
//     http::StatusCode,
// };
// use serde_json::json;
// use std::net::SocketAddr;

use context::build_context;
use database::{count_disk_events, setup_mongo};
use error::{get_error_message, Result};

#[tokio::main]
async fn main() {
    // initialize logging, configured by environment
    env_logger::init();
    // run the application
    match do_main().await {
        Err(e) => eprintln!("Error: {}", get_error_message(e)),
        Ok(_) => {}
    }
}

async fn do_main() -> Result<()> {
    let context = build_context().expect("Unable to build application context");

    println!("{:?}", context);
    println!("{}", context.get_mongo_url());

    let client = setup_mongo(&context)
        .await
        .expect("Unable to initialize MongoDB client");
    let count = count_disk_events(&context, &client).await?;
    println!("{}", count);

    Ok(())
}

//     // Create a MongoDB client
//     let mongo_client = setup_mongo().await.expect("Failed to initialize MongoDB client");

//     // Build our application with a route
//     let app = Router::new()
//         .route("/health", get(health_handler))
//         .with_state(mongo_client);

//     // Run the server
//     let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
//     println!("Listening on {}", addr);
//     axum::Server::bind(&addr)
//         .serve(app.into_make_service())
//         .await
//         .unwrap();
// }

// async fn health_handler(client: axum::extract::State<Client>) -> impl IntoResponse {
//     let database = client.database("your_database_name");
//     let collection = database.collection("disk_event");

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
// }
