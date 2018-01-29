extern crate futures;
extern crate futures_cpupool;

use std::thread;
use std::time;
use futures::prelude::*;
use futures::future;
use futures::Future;
// use futures_cpupool::CpuPool;

fn do_pool() {

    let pool = futures_cpupool::CpuPool::new(4);

    // Execute some work on the thread pool, optionally closing over data.
    let a = pool.spawn(long_running_future(2));
    let b = pool.spawn(long_running_future(100));

    // Express some further computation once the work is completed on the thread
    // pool.
    let c = a.join(b).map(|(a, b)| a + b).wait().unwrap();

    // Print out the result
    println!("{:?}", c);
}

fn long_running_future(i: i32) -> Future<String, i32> {
    //info!("CPU_IN");
    let mut y = "X".to_string();
    let mut e = 10;
    for x in 0..1000 {
        y = format!("Value: {}", x);
        e = e + y.len();
    }

    let sleep_time = time::Duration::from_secs(5);
    thread::sleep(sleep_time);
    "asdf".to_string()
}
