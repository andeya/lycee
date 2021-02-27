use std::error::Error;

use modify::Modify;

mod modify;
mod kvdb;
pub mod inner;
mod allocator;
#[macro_use]
mod cache;
mod crc64;
mod mmap;
mod cmd;

/// Storage represents the internal-facing server part of TinyKV, it handles sending and receiving from other
/// TinyKV nodes. As part of that responsibility, it also reads and writes data to disk (or semi-permanent memory).
trait Storage {
    fn start(&self) -> Result<(), Box<dyn Error>>;
    fn stop(&self) -> Result<(), Box<dyn Error>>;
    fn write(&self, batch: Vec<Modify>) -> Result<(), Box<dyn Error>>;
    fn reader(&self) -> Result<Box<dyn StorageReader>, Box<dyn Error>>;
}

trait StorageReader {
    fn get_cf(&self, cf: String, key: Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>>;
    fn iter_cf(&self, cf: String) -> dyn Iterator<Item=dyn DBItem>;
    fn close(&self);
}

trait DBItem {
    /// Key returns the key.
    fn key(&self) -> Vec<u8>;
    /// KeyCopy returns a copy of the key of the item, writing it to dst slice.
    /// If nil is passed, or capacity of dst isn't sufficient, a new slice would be allocated and
    /// returned.
    fn key_copy(&self, dst: Vec<u8>) -> Vec<u8>;
    /// Value retrieves the value of the item.
    fn value(&self) -> Result<Vec<u8>, Box<dyn Error>>;
    /// ValueSize returns the size of the value.
    fn value_size(&self) -> i32;
    /// ValueCopy returns a copy of the value of the item from the value log, writing it to dst slice.
    /// If nil is passed, or capacity of dst isn't sufficient, a new slice would be allocated and
    /// returned.
    fn value_copy(&self, dst: Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>>;
}

#[cfg(test)]
mod tests {
    use crate::kv::storage::cache::cache_s;
    use crate::kv::storage::inner::node_s;

    #[test]
    fn is_it_work() {
        assert_eq!(1 + 2, 3);
    }

    #[test]
    fn test_cache_s_new() {
        let cache = cache_s::new();
        for (i, emem) in cache.hash.iter().enumerate() {
            println!("cache.hash {}:{:?}", i, emem);
        }
        let nodes = array_init!(node_s { prev: None, next: None };10);
        for (i, emem) in nodes.iter().enumerate() {
            println!("nodes {}:{:?}", i, emem);
        }
    }
}
