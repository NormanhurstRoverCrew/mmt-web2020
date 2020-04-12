#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
use std::path::PathBuf;

pub fn init() {
    rocket::ignite()
        //.mount("/", routes![catch_unauthorized, catch_authorized])
        .launch();
}
