extern crate futures;
extern crate hyper;
extern crate pretty_env_logger;
extern crate net2;
extern crate tokio_core;
extern crate serde;
extern crate serde_json;

use std::{thread, time};
use std::sync::Arc;
use std::net::SocketAddr;
use futures::Stream;
use net2::unix::UnixTcpBuilderExt;
use tokio_core::reactor::Core;
use tokio_core::net::TcpListener;
use futures::future::FutureResult;
use hyper::{Get, StatusCode};
use hyper::header::ContentLength;
use hyper::server::{Http, Service, Request, Response};

pub fn start_server(nb_instances: usize, addr: &str) {
    let addr = addr.parse().unwrap();

    let protocol = Arc::new(Http::new());
    {
        for id in 1..nb_instances {
            let protocol = protocol.clone();
            thread::spawn(move || serve(&addr, &protocol, id));
        }
    }
    serve(&addr, &protocol, 0);
}

fn serve(addr: &SocketAddr, protocol: &Http, id: usize) {
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let listener = net2::TcpBuilder::new_v4()
        .unwrap()
        .reuse_port(true)
        .unwrap()
        .bind(addr)
        .unwrap()
        .listen(128)
        .unwrap();
    let listener = TcpListener::from_listener(listener, addr, &handle).unwrap();
    core.run(listener.incoming().for_each(|(socket, addr)| {
        protocol.bind_connection(&handle, socket, addr, Echo { id: id });
        Ok(())
    })).unwrap();
}


#[derive(Serialize, Deserialize)]
struct Address {
    street: String,
    city: String,
    count: usize,
}

fn cpu_intensive_work() -> String {
    //info!("CPU_IN");
    let mut y = "X".to_string();
    let mut e = 10;
    for x in 0..10000 {
        y = format!("Value: {}", x);
        e = e + y.len();
    }
    let address = Address {
        street: "10 Downing Street".to_owned(),
        city: y.to_owned(),
        count: e,
    };

    // let sleep_time = time::Duration::from_secs(5);
    // thread::sleep(sleep_time);
    let j = serde_json::to_string(&address).unwrap();
    //info!("CPU_OUT");
    return j;
}

#[derive(Clone, Copy)]
struct Echo {
    id: usize,
}

impl Service for Echo {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = FutureResult<Response, hyper::Error>;

    fn call(&self, req: Request) -> Self::Future {
        futures::future::ok(match (req.method(), req.path()) {
            (&Get, "/data") => {
                println!("here==== {}", self.id);
                let b = cpu_intensive_work().into_bytes();
                Response::new()
                    .with_header(ContentLength(b.len() as u64))
                    .with_body(b)
            }
            _ => Response::new().with_status(StatusCode::NotFound),
        })
    }
}
