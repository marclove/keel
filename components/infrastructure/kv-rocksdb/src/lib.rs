#![cfg(target_arch = "wasm32")]

wit_bindgen::generate!({
    world: "kv-adapter",
    path: "../../../wit",
});

use exports::keel::infrastructure::kv::{Guest, KvValue, KvError, ScanResult};

struct Component;

impl Guest for Component {
    fn get(_key: String) -> Result<Option<KvValue>, KvError> {
        todo!("get not yet implemented")
    }

    fn set(_key: String, _value: KvValue) -> Result<(), KvError> {
        todo!("set not yet implemented")
    }

    fn set_with_ttl(_key: String, _value: KvValue, _ttl_seconds: u32) -> Result<(), KvError> {
        todo!("set_with_ttl not yet implemented")
    }

    fn delete(_key: String) -> Result<bool, KvError> {
        todo!("delete not yet implemented")
    }

    fn exists(_key: String) -> Result<bool, KvError> {
        todo!("exists not yet implemented")
    }

    fn increment(_key: String, _delta: i64) -> Result<i64, KvError> {
        todo!("increment not yet implemented")
    }

    fn expire(_key: String, _ttl_seconds: u32) -> Result<bool, KvError> {
        todo!("expire not yet implemented")
    }

    fn scan(_pattern: String, _cursor: Option<String>, _limit: Option<u32>) -> Result<ScanResult, KvError> {
        todo!("scan not yet implemented")
    }
}

export!(Component);
