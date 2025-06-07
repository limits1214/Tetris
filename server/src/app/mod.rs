use axum::Router;
use listenfd::ListenFd;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    app::config::AppConfig, controller::init_controller_router,
    controller_ws::init_controller_ws_router,
};

pub mod config;
pub mod state;

pub async fn app_start() {
    AppConfig::init().await;
    let _guard = init_tracing();
    init_axum().await;
}

fn init_tracing() -> WorkerGuard {
    let file_appender = tracing_appender::rolling::daily("logs", "client.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "tetris_server=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::fmt::layer().with_writer(non_blocking))
        .init();
    guard
}

async fn init_axum() {
    let listener = init_listenfd(4000).await;
    let router_controller = init_controller_router().await;
    let router_controller_ws = init_controller_ws_router().await;
    let router = Router::new()
        .merge(router_controller)
        .merge(router_controller_ws);
    axum::serve(listener, router)
        .with_graceful_shutdown(init_shutdown_signal())
        .await
        .unwrap();
}

async fn init_listenfd(server_port: u32) -> tokio::net::TcpListener {
    // reload
    // https://github.com/tokio-rs/axum/tree/main/examples/auto-reload
    let mut listenfd = ListenFd::from_env();
    let listener = match listenfd.take_tcp_listener(0).unwrap() {
        Some(listener) => {
            listener.set_nonblocking(true).unwrap();
            tracing::info!("reload bind_ip: {:?}", listener.local_addr());
            tokio::net::TcpListener::from_std(listener).unwrap()
        }
        None => {
            let bind_ip = format!("0.0.0.0:{}", server_port);
            tracing::info!("bind_ip: {}", bind_ip);
            tokio::net::TcpListener::bind(bind_ip).await.unwrap()
        }
    };
    listener
}

/// 그레이스풀 셧다운
/// https://github.com/tokio-rs/axum/blob/main/examples/graceful-shutdown/src/main.rs
async fn init_shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };
    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    tokio::select! {
        _ = ctrl_c => {
            println!("shoutdown ctrl_c")
        },
        _ = terminate => {
            println!("shoutdown terminate")
        },
    }
}
