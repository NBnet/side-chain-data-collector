use std::env;

use anyhow::Result;
use clap::Parser;
use fetcher::Fetcher;

mod fetcher;

#[derive(Debug, Parser)]
pub struct Command {
    #[clap(short, long, env = "SIDE_CHAIN_DATA_PATH")]
    path: Option<String>,

    #[clap(short, long, env)]
    url: Option<String>,

    #[clap(short = 'd', long)]
    daemon: bool,
}

impl Command {
    pub async fn execute(self) -> Result<()> {
        let path = self.path.unwrap_or("/tmp/side_chain_data".to_string());
        let url = self
            .url
            .unwrap_or("https://tendermint-sidechain-testnet.expchain.ai".to_string());
        let mut fetcher = Fetcher::new(path.into(), &url)?;

        if self.daemon {
            nix::unistd::daemon(false, true)?;
        }

        fetcher.run().await
    }
}
#[tokio::main]
async fn main() -> Result<()> {
    log::info!("scd started");

    if env::var_os("RUST_BACKTRACE").is_none() {
        env::set_var("RUST_BACKTRACE", "full");
    }
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "info");
    }

    env_logger::init();

    let cmd = Command::parse();

    cmd.execute().await
}
