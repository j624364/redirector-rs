mod cmd_args;

use clap::Parser;
use cmd_args::CmdArgs;

async fn redirect_root(
    axum::extract::State(cmd_args): axum::extract::State<CmdArgs>,
) -> impl axum::response::IntoResponse {
    axum::response::Redirect::temporary(&cmd_args.redirect_address)
}

#[tokio::main]
async fn main() {
    let args = CmdArgs::parse();
    println!("{} -> {}", args.host_address, args.redirect_address);

    let listener = tokio::net::TcpListener::bind(&args.host_address)
        .await
        .unwrap();
    let app = axum::Router::new()
        .route("/", axum::routing::get(redirect_root))
        .with_state(args);
    axum::serve(listener, app).await.unwrap();
}
