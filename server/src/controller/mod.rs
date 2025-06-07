use axum::Router;
use tower_http::trace::TraceLayer;

use crate::{app::state::ArcAppState, controller::test::test_router};

pub mod test;

pub async fn init_controller_router() -> Router {
    let arc_app_state = ArcAppState::new().await;
    Router::new()
        .merge(test_router(arc_app_state.clone()))
        .layer(TraceLayer::new_for_http())
        .with_state(arc_app_state)
}
