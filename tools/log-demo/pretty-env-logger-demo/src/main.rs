extern crate pretty_env_logger;
#[macro_use] extern crate log;

fn main() {
    pretty_env_logger::init();

    trace!("a trace example");
    debug!("debugging");
    info!("such information");
    warn!("o_O");
    error!("boom");
}