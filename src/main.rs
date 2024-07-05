mod cmd_args;

use clap::Parser;
use cmd_args::CmdArgs;

#[derive(Debug, Clone)]
struct AppState {
    pub redirect_address: String,
}

async fn redirect_root(
    axum::extract::State(app_state): axum::extract::State<AppState>,
) -> impl axum::response::IntoResponse {
    axum::response::Redirect::temporary(&app_state.redirect_address)
}

#[tokio::main]
async fn main() {
    let args = CmdArgs::parse();

    if args.print_listening_message.unwrap_or(false) {
        println!("{} -> {}", args.host_address, args.redirect_address);
    }

    let (app_state, host_address) = args.to_app_state();

    let app = axum::Router::new()
        .route("/", axum::routing::get(redirect_root))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(host_address).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
