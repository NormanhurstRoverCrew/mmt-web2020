#![feature(decl_macro, proc_macro_hygiene)]

#[macro_use]
extern crate mongodb;

extern crate juniper;

pub mod db;
pub mod graphql;
pub mod models;
pub mod routes;
