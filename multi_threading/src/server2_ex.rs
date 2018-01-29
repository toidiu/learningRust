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
use tokio_core::net::{TcpListener, TcpStream};
use futures::future::FutureResult;
use futures::task;
use hyper::{Get, StatusCode};
use hyper::header::ContentLength;
use hyper::server::{Http, Service, Request, Response};
use std::sync::Mutex;

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
    let listener = net2::TcpBuilder::new_v4().unwrap()
    .reuse_port(true).unwrap()
    .bind(addr).unwrap()
    .listen(128).unwrap();
    let listener = TcpListener::from_listener(listener, addr, &handle).unwrap();
    core.run(listener.incoming().for_each(|(socket, addr)| {
        protocol.bind_connection(&handle, socket, addr, Echo{ id : id});
        Ok(())
    })).unwrap();
}


// ====================================


struct ThreadData {
    entries: Vec<(TcpStream, SocketAddr)>,
    task: Option<task::Task>,
}

impl ThreadData {
 pub fn new() -> Arc<Mutex<ThreadData>> {
        Arc::new(Mutex::new(ThreadData {
            entries: Vec::new(),
            task: None,
        }))
    }
}

pub fn start_better_server(addr: &str, num_thread: usize) {

//// some init
    let addr: SocketAddr = addr.parse().unwrap();
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    // let protocol = Arc::new(Http::new());

//// have a vec of thread data that can handle the incoming request
    let threads = make_service_threads(num_thread);


//// build the listener
    let listener = net2::TcpBuilder::new_v4().unwrap()
        .reuse_port(true).unwrap()
        .bind(&addr).unwrap()
        .listen(128).unwrap();
    let listener = TcpListener::from_listener(listener, &addr, &handle).unwrap();


    let mut counter = 0;
//// on main thread have a core run
    core.run(listener.incoming().for_each(|(socket, addr)| {

        //// for each incoming request add the data to thread round-robin style
        let ref entry = threads[counter];
        {
            let mut entry = entry.lock().unwrap();
            entry.entries.push((socket, addr));

            if let Some(task) = entry.task.take() {
                task.unpark();
            }
        }
        counter += 1;
        if counter >= num_thread {
            counter = 0;
        }
        Ok(())
    })).unwrap();
}

fn make_service_threads(num_thread: usize) -> Vec<Mutex<ThreadData>> {

/// here we will create num_thread threads and have a ThreadData associated with each

/// we also need a Core per each thread to poll the entries(connections)

/// the handling will happen by the hyper Server implementation

// protocol.bind_connection(&handle, socket, addr, Echo{ id : 0});
    Vec::new()
}









#[derive(Clone, Copy)]
struct Echo{
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
                    // let b = cpu_intensive_work().into_bytes();

                    for x in 0..10000 {
                        let y = format!("Value: {}", x);
                    }
                    // let sleep_time = time::Duration::from_secs(5);
                    // thread::sleep(sleep_time);
                    let b = "";

                    Response::new()
                        .with_header(ContentLength(b.len() as u64))
                        .with_body(b)
                }
                _ => Response::new().with_status(StatusCode::NotFound),
            })
    }
}
