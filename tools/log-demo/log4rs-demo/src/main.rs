use log::{error, info, warn};
use log4rs;

fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    info!("booting up");
    error!("log error");
    warn!("log warn");
    // ...
}
