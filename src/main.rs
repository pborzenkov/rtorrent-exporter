use clap::Parser;
use prometheus_client::registry::Registry;
use reqwest::Url;
use rtorrent_xmlrpc_bindings::Server;
use std::net::SocketAddr;
use tokio::net::TcpListener;

mod collector;
mod exporter;

#[derive(Parser)]
#[clap(author, version, about)]
struct Args {
    #[clap(
        short,
        long,
        help = "Address to listen on",
        default_value = "127.0.0.1:9091"
    )]
    address: SocketAddr,

    #[clap(
        short,
        long,
        help = "RTorrent RPC URL",
        default_value = "http://127.0.0.1:5000/RPC2"
    )]
    rtorrent: Url,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    let rtorrent = Server::new(args.rtorrent.as_str());
    let mut registry = <Registry>::default();

    collector::register_metrics(&mut registry, rtorrent);

    let exporter = exporter::get_router(registry);
    let listener = TcpListener::bind(&args.address).await.unwrap();

    axum::serve(listener, exporter).await.unwrap();
}
