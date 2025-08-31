//! Placeholder RocksDB KV adapter implementing `kv` WIT interface.
//! To be completed with real RocksDB bindings suitable for WASM.

wit_bindgen::generate!({
    world: "kv-adapter",
    path: "../../../wit",
});

use exports::keel::infrastructure::kv as wit_kv;

struct Adapter;

fn op_err(msg: &str) -> wit_kv::KvError {
    wit_kv::KvError::OperationFailed(msg.to_string())
}

impl wit_kv::Guest for Adapter {
    fn get(_key: String) -> Result<Option<wit_kv::KvValue>, wit_kv::KvError> {
        Err(op_err("not implemented"))
    }

    fn set(_key: String, _value: wit_kv::KvValue) -> Result<(), wit_kv::KvError> {
        Err(op_err("not implemented"))
    }

    fn set_with_ttl(_key: String, _value: wit_kv::KvValue, _ttl_seconds: u32) -> Result<(), wit_kv::KvError> {
        Err(op_err("not implemented"))
    }

    fn delete(_key: String) -> Result<bool, wit_kv::KvError> { Err(op_err("not implemented")) }
    fn exists(_key: String) -> Result<bool, wit_kv::KvError> { Err(op_err("not implemented")) }

    fn increment(_key: String, _delta: i64) -> Result<i64, wit_kv::KvError> {
        Err(op_err("not implemented"))
    }

    fn expire(_key: String, _ttl_seconds: u32) -> Result<bool, wit_kv::KvError> {
        Err(op_err("not implemented"))
    }

    fn scan(_pattern: String, _cursor: Option<String>, _limit: Option<u32>) -> Result<wit_kv::ScanResult, wit_kv::KvError> {
        Err(op_err("not implemented"))
    }
}

export!(Adapter);

