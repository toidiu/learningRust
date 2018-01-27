#![allow(unused_imports)]

#[macro_use] extern crate log;
extern crate env_logger;
extern crate log4rs;

// use log::LevelFilter;
// use log4rs::append::file::FileAppender;
// use log4rs::encode::pattern::PatternEncoder;
// use log4rs::config::{Appender, Config, Root};
use log4rs::file::Deserializers;



fn main() {
    // env_logger::init();

    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    trace!("trace");
    debug!("debug");
    info!("info");
    warn!("warn");
    error!("error");

}
