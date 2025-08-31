#![deny(unsafe_code)]
//! # sqllite Adapter
//!
//! ## Implementation Status
//!

mod bindings {
    #![allow(unsafe_code)]
    wit_bindgen::generate!({
        world: "sql-adapter",
        path: "../../../wit",
    });
}
