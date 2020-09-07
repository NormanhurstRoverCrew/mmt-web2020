#![feature(vec_remove_item)]

extern crate mongodb;

#[macro_use]
extern crate juniper;

pub mod db;
pub mod graphql;
pub mod models;
pub mod routes;
pub mod stripe;
