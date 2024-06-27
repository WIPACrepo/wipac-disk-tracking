// events.rs

use gotham::{router::response::StaticResponseExtender, state::StateData};
use gotham_restful::{create, read, search, Resource, Success};
use log::{error, info};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Resource, Serialize)]
#[resource(create_event)]
#[resource(find_event_by_query)]
#[resource(get_event_by_id)]
pub struct EventResource {
    pub id: u64,
}

#[derive(Serialize)]
pub struct EventResources {
    pub events: Vec<EventResource>,
}

// --------------------------------------------------------------------------------------------------------------------

#[derive(Clone, Deserialize, StateData, StaticResponseExtender)]
struct EventBody {
    serial_number: String,
}

#[create]
fn create_event(body: EventBody) -> Success<EventResource> {
    info!("create_event()");
    EventResource { id: 0 }.into()
}

// --------------------------------------------------------------------------------------------------------------------

#[derive(Clone, Deserialize, StateData, StaticResponseExtender)]
struct EventSearchQuery {
    serial_number: String,
}

#[search]
fn find_event_by_query(query: EventSearchQuery) -> Success<EventResources> {
    info!("find_event_by_query()");
    EventResources {
        events: Vec::new(),
    }.into()
}

// --------------------------------------------------------------------------------------------------------------------

#[read]
fn get_event_by_id(id: u64) -> Success<EventResource> {
    info!("get_event_by_id()");
    EventResource { id }.into()
}
