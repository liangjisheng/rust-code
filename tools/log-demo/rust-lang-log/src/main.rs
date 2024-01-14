use log::{error, warn, info, debug, trace};

struct Connection {
    port: i32,
    speed: f64,
}

fn main() {
    let conn_info = Connection { port: 40, speed: 3.20 };

    info!("Connected to port {} at {} Mb/s", conn_info.port, conn_info.speed);
    info!(target: "connection_events", "Successful connection, port: {}, speed: {}",
      conn_info.port, conn_info.speed);
}
