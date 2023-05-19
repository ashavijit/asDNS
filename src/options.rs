use clap::Parser;
use std::net::SocketAddr;

#[derive(Parser, Clone, Debug)]
pub struct Options {
    #[clap(long, short, default_value = "0.0.0.0:1053", env = "DNSFUN_UDP")]
    pub udp: vec<SocketAddr>,

    #[clap(long, short, env = "DNSFUN_TCP")]
    pub tcp: vec<SocketAddr>,

    #[clap(long, short, default_value = "google.com", env = "DNSFUN_DOMAIN")]
    pub domain: String,
}
