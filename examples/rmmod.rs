extern crate kmod;
#[macro_use] extern crate log;
extern crate env_logger;

use std::fs;
use std::env;

fn main() {
    env_logger::init();

    let ctx = kmod::Context::new().expect("kmod ctx failed");

    let filename = env::args().nth(1).expect("missing argument");

    let module = match fs::metadata(&filename) {
        Ok(_) => ctx.module_new_from_path(&filename).expect("new_from_path failed"),
        Err(_) => ctx.module_new_from_name(&filename).expect("new_from_name failed"),
    };

    info!("got module: {:?}", module.name());

    module.remove_module(0).expect("remove_module failed");
}
