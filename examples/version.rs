extern crate renderdoc_api;

use std::thread;
use std::time::Duration;

fn main() {
    let ctx = renderdoc_api::Context::new();
    match ctx {
        Some(ctx) => {
            let (x, y, z) = ctx.get_api_version();
            println!("Found renderdoc {}.{}.{}", x, y, z);
        }
        None => {
            println!("Couldn't find renderdoc");
        }
    }
    thread::sleep(Duration::from_secs(5));
}
