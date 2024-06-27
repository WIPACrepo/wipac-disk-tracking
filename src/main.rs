// main.rs

pub mod routes;

use gotham::{
    router::{build_simple_router, builder::DrawRoutes, Router},
    state::State,
};
use log::info;

const HELLO_WORLD: &str = "Hello World!\n";

pub fn say_hello(state: State) -> (State, &'static str) {
    (state, HELLO_WORLD)
}

fn build_router() -> Router {
    build_simple_router(|route| {
        route.delegate("/api/v1").to_router(routes::v1::router());
    })
}

pub fn main() {
    // initialize logging, configured by environment
    env_logger::init();
    // start the service
    let addr = "0.0.0.0:8080";
    info!("Listening for requests at http://{}", addr);
    // gotham::start(addr, || Ok(say_hello)).unwrap();
    let _ = gotham::start(addr, build_router());
}

// --------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use gotham::hyper::StatusCode;
    use gotham::test::TestServer;

    #[test]
    fn test_always_succeed() {
        assert_eq!(true, true)
    }

    #[test]
    fn test_say_hello() {
        let test_server = TestServer::new(|| Ok(say_hello)).unwrap();
        let response = test_server
            .client()
            .get("http://localhost")
            .perform()
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.read_utf8_body().unwrap();
        assert_eq!(body, HELLO_WORLD);
    }
}
