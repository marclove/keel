#![deny(unsafe_code)]
//! # Cloudflare DO SQL Adapter

mod bindings {
    #![allow(unsafe_code)]
    wit_bindgen::generate!({
        world: "sql-adapter",
        path: "../../../wit",
    });
}
