use log::{info, warn};

fn main() {
    env_logger::init();
    info!("starting up. Is this a info???");
    warn!("oops, nothing implemented! Is this a warn???");
}
