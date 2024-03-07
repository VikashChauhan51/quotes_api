mod app;
use log::{info, trace,LevelFilter};
use env_logger::Builder;

fn main() {
    Builder::new()
        .filter(None, LevelFilter::Info)
        .init();

    trace!("quotes starting...");
    info!("quotes starting...");
    println!("Hello, world!");
    info!("done.")
}
