use axum::Router;

use crate::{app::state::ArcAppState, controller_ws::test::test_ws_router};

pub mod test;

pub async fn init_controller_ws_router() -> Router {
    let arc_app_state = ArcAppState::new().await;
    Router::new()
        .merge(test_ws_router(arc_app_state.clone()))
        .with_state(arc_app_state)
}
