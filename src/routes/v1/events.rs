// events.rs

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use axum_extra::response::ErasedJson;
use chrono::Utc;
use serde::Serialize;
use uuid::Uuid;

use crate::context::ApplicationContext;
use crate::database::{find_disk_events_serial_number, find_disk_events_uuid, save_disk_event};
use crate::event::{DiskEvent, Event};
use crate::smartctl::SmartCtl;

#[derive(Serialize)]
struct DiskEventCreatedResponse {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Serialize)]
struct DiskEventFindResponse {
    pub events: Vec<DiskEvent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

pub async fn get_events(
    State(context): State<ApplicationContext>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    // if we can parse the provided id as a UUID
    if let Ok(uuid) = Uuid::parse_str(&id) {
        // search the database by UUID
        let (status, response) = find_events_by_uuid(context, uuid).await;
        return (status, ErasedJson::pretty(response));
    }

    // look the events up by serial_number
    let (status, response) = find_events_by_serial_number(context, id).await;
    (status, ErasedJson::pretty(response))
}

pub async fn post_closed(
    State(context): State<ApplicationContext>,
    Json(smartctl): Json<SmartCtl>,
) -> impl IntoResponse {
    let (status, response) = create_event(context, smartctl, Event::CLOSED).await;
    (status, ErasedJson::pretty(response))
}

pub async fn post_formatted(
    State(context): State<ApplicationContext>,
    Json(smartctl): Json<SmartCtl>,
) -> impl IntoResponse {
    let (status, response) = create_event(context, smartctl, Event::FORMATTED).await;
    (status, ErasedJson::pretty(response))
}

pub async fn post_opened(
    State(context): State<ApplicationContext>,
    Json(smartctl): Json<SmartCtl>,
) -> impl IntoResponse {
    let (status, response) = create_event(context, smartctl, Event::OPENED).await;
    (status, ErasedJson::pretty(response))
}

pub async fn post_sighted(
    State(context): State<ApplicationContext>,
    Json(smartctl): Json<SmartCtl>,
) -> impl IntoResponse {
    let (status, response) = create_event(context, smartctl, Event::SIGHTED).await;
    (status, ErasedJson::pretty(response))
}

async fn create_event(
    context: ApplicationContext,
    smartctl: SmartCtl,
    event: Event,
) -> (StatusCode, DiskEventCreatedResponse) {
    // ensure we can extract a serial number
    let serial_number = if let Some(serial) = smartctl.get_serial() {
        serial
    } else {
        return (
            StatusCode::UNPROCESSABLE_ENTITY,
            DiskEventCreatedResponse {
                status: "error".to_string(),
                message: Some("provided smartctl data is missing key 'serial_number'".to_string()),
            },
        );
    };

    // extract a creation date, or just use the current one
    let date_created = if let Some(date) = smartctl.get_date_created() {
        date
    } else {
        // the user provided smartctl data without a date, so we'll use the current one
        Utc::now().to_rfc3339().replace("+00:00", "Z")
    };

    // create the disk event
    let disk_event = DiskEvent {
        uuid: Uuid::new_v4(),
        date_created,
        serial_number,
        event,
        smartctl,
    };

    // store the disk event in the database
    match save_disk_event(&context, disk_event).await {
        Ok(_) => (
            StatusCode::CREATED,
            DiskEventCreatedResponse {
                status: "ok".to_string(),
                message: None,
            },
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            DiskEventCreatedResponse {
                status: "error".to_string(),
                message: Some(format!("{:?}", e)),
            },
        ),
    }
}

async fn find_events_by_uuid(
    context: ApplicationContext,
    uuid: Uuid,
) -> (StatusCode, DiskEventFindResponse) {
    match find_disk_events_uuid(&context, uuid).await {
        Ok(events) => (
            StatusCode::OK,
            DiskEventFindResponse {
                events,
                message: None,
            },
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            DiskEventFindResponse {
                events: Vec::new(),
                message: Some(format!("{:?}", e)),
            },
        ),
    }
}

async fn find_events_by_serial_number(
    context: ApplicationContext,
    serial_number: String,
) -> (StatusCode, DiskEventFindResponse) {
    match find_disk_events_serial_number(&context, serial_number).await {
        Ok(events) => (
            StatusCode::OK,
            DiskEventFindResponse {
                events,
                message: None,
            },
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            DiskEventFindResponse {
                events: Vec::new(),
                message: Some(format!("{:?}", e)),
            },
        ),
    }
}
