use crate::AppState;
use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[command(version, about)]
pub struct CmdArgs {
    pub host_address: String,
    pub redirect_address: String,

    #[arg(long)]
    pub maintain_path: Option<bool>,
    #[arg(long)]
    pub maintain_params: Option<bool>,
    #[arg(long)]
    pub print_listening_message: Option<bool>,
}

impl CmdArgs {
    pub fn to_app_state(self) -> (AppState, String) {
        (
            AppState {
                redirect_address: self.redirect_address,
                maintain_path: self.maintain_path.unwrap_or(true),
                maintain_params: self.maintain_params.unwrap_or(true),
            },
            self.host_address,
        )
    }
}
