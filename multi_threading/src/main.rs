#![allow(deprecated)]
#![allow(unused)]

extern crate rayon;

extern crate futures;
extern crate hyper;
extern crate pretty_env_logger;
extern crate net2;
extern crate tokio_core;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod rayon_ex;
mod server_ex;
mod server2_ex;
mod channels_ex;
// mod cpu_pool_ex;

fn main() {
    // rayon_ex::rayon_main();
    // server_ex::start_server(4, "0.0.0.0:8080");
    // server2_ex::start_server(4, "0.0.0.0:8080");
    server2_ex::start_better_server("0.0.0.0:8080", 4);
    // channels_ex::ch();
}


