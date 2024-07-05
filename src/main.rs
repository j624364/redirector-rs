mod cmd_args;

use clap::Parser;

fn main() {
    let args = cmd_args::CmdArgs::parse();
    println!("{} -> {}", args.host_address, args.redirect_address);
}
