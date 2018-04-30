extern crate tokio;
use tokio::runtime::Runtime;
use tokio::prelude::*;
use tokio::timer::Delay;
use std::time::{Duration, Instant};
use std::ops::Add;

fn main() {
    let mut rt = Runtime::new().unwrap();
    let soon = Instant::now()
        .add(Duration::new(2,0));

    let delay = Delay::new(soon);
    let lazy = future::lazy(|| {
        println!("now running on a worker thread");
        Ok(())
    });
    println!("starting up");
    
    let f = delay.then(|_|{lazy});

    rt.spawn(f);
    rt.shutdown_on_idle()
        .wait().unwrap();
    println!("done");
}
