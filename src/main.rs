mod cmd_args;

use axum::{
    extract::{Path, Query, State},
    response::{IntoResponse, Redirect},
    routing,
};
use clap::Parser;
use cmd_args::CmdArgs;

#[derive(Debug, Clone)]
struct AppState {
    pub redirect_address: String,
}

fn get_custom_address(
    app_state: &AppState,
    path: Option<&str>,
    params: Vec<(String, String)>,
) -> String {
    let mut address = String::with_capacity(app_state.redirect_address.len() + 256);
    address.push_str(&app_state.redirect_address);

    if let Some(path) = path {
        address.push('/');
        address.push_str(path);
    }

    for (i, (key, value)) in params.iter().enumerate() {
        address.push(if i == 0 { '?' } else { '&' });
        address.push_str(key);
        address.push('=');
        address.push_str(value);
    }

    address
}

async fn redirect_root(
    State(app_state): State<AppState>,
    Query(params): Query<Vec<(String, String)>>,
) -> impl IntoResponse {
    if params.is_empty() {
        Redirect::temporary(&app_state.redirect_address)
    } else {
        Redirect::temporary(&get_custom_address(&app_state, None, params))
    }
}

async fn redirect(
    State(app_state): State<AppState>,
    Path(path): Path<String>,
    Query(params): Query<Vec<(String, String)>>,
) -> impl IntoResponse {
    Redirect::temporary(&get_custom_address(&app_state, Some(&path), params))
}

#[tokio::main]
async fn main() {
    let args = CmdArgs::parse();

    if args.print_listening_message.unwrap_or(false) {
        println!("{} -> {}", args.host_address, args.redirect_address);
    }

    let (app_state, host_address) = args.to_app_state();

    let app = axum::Router::new()
        .route("/", routing::get(redirect_root))
        .route("/*path", routing::get(redirect))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(host_address).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
