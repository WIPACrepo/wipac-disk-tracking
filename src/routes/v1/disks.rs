// disks.rs

use gotham::{router::response::StaticResponseExtender, state::StateData};
use gotham_restful::{endpoint, gotham::hyper::Method, read, search, Resource, Success};
use log::{error, info};
use serde::{Deserialize, Serialize};

use crate::routes::v1::{EventResource, EventResources};

#[derive(Resource, Serialize)]
#[resource(get_disk_by_id)]
#[resource(get_event_by_disk_id_and_event_id)]
#[resource(get_events_by_disk_id)]
#[resource(find_disk_by_query)]
pub struct DiskResource {
    pub id: u64,
}

#[derive(Serialize)]
pub struct DiskResources {
    pub disks: Vec<DiskResource>,
}

// --------------------------------------------------------------------------------------------------------------------

#[derive(Clone, Deserialize, StateData, StaticResponseExtender)]
struct DiskSearchQuery {
    serial_number: String,
}

#[search]
fn find_disk_by_query(query: DiskSearchQuery) -> Success<DiskResources> {
    info!("find_disk_by_query()");
    DiskResources {
        disks: Vec::new()
    }.into()
}

// --------------------------------------------------------------------------------------------------------------------

#[read]
fn get_disk_by_id(disk_id: u64) -> Success<DiskResource> {
    info!("get_disk_by_id()");
    DiskResource { id: disk_id }.into()
}

// --------------------------------------------------------------------------------------------------------------------

#[derive(Clone, Deserialize, StateData, StaticResponseExtender)]
struct DiskAndEvent {
    disk_id: u64,
    event_id: u64,
}

#[endpoint(
    uri = ":disk_id/events/:event_id",
    method = "Method::GET",
    params = false,
    body = false
)]
fn get_event_by_disk_id_and_event_id(disk_and_event: DiskAndEvent) -> Success<EventResource> {
    info!("get_event_by_disk_id_and_event_id()");
    EventResource { id: disk_and_event.event_id }.into()
}

// --------------------------------------------------------------------------------------------------------------------

#[derive(Clone, Deserialize, StateData, StaticResponseExtender)]
struct Disk {
    disk_id: u64,
}

#[endpoint(
    uri = ":disk_id/events",
    method = "Method::GET",
    params = false,
    body = false
)]
fn get_events_by_disk_id(disk: Disk) -> Success<EventResources> {
    info!("get_events_by_disk_id()");
    EventResources {
        events: Vec::new()
    }.into()
}
