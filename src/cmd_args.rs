use clap::Parser;

#[derive(Debug, Clone, Parser)]
pub struct CmdArgs {
    pub host_address: String,
    pub redirect_address: String,
}
