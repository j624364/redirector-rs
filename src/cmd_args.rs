use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[command(version, about)]
pub struct CmdArgs {
    pub host_address: String,
    pub redirect_address: String,

    #[arg(long)]
    pub print_listening_message: Option<bool>,
}
