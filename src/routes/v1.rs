// v1.rs

pub mod disks;
pub mod events;

pub use disks::{DiskResource, DiskResources};
pub use events::{EventResource, EventResources};

use gotham::router::{build_simple_router, Router};
use gotham_restful::DrawResources;

pub fn router() -> Router {
    build_simple_router(|route| {
        route.resource::<DiskResource>("disks");
        route.resource::<EventResource>("events");
    })
}
