use clap::Parser;

#[derive(Parser)]
pub struct CmdArgs {
    pub host_address: String,
    pub redirect_address: String,
}
