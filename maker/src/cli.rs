use anyhow::Result;
use clap::Parser;
use reqwest::Url;
use std::env::current_dir;
use std::net::SocketAddr;
use std::path::PathBuf;

#[derive(Parser)]
pub struct Opts {
    /// The address to listen on for the lightning and dlc peer2peer API.
    #[clap(long, default_value = "0.0.0.0:19045")]
    pub p2p_address: SocketAddr,

    /// The IP address to listen on for the HTTP API.
    #[clap(long, default_value = "0.0.0.0:18000")]
    pub http_address: SocketAddr,

    /// Where to permanently store data, defaults to the current working directory.
    #[clap(long)]
    data_dir: Option<PathBuf>,

    /// The HTTP address for the orderbook.
    #[clap(long, default_value = "http://localhost:8000")]
    pub orderbook: Url,
}

impl Opts {
    // use this method to parse the options from the cli.
    pub fn read() -> Opts {
        Opts::parse()
    }

    pub fn data_dir(&self) -> Result<PathBuf> {
        let data_dir = match self.data_dir.clone() {
            None => current_dir()?.join("data"),
            Some(path) => path,
        }
        .join("maker");

        Ok(data_dir)
    }
}
