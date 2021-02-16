use std::error::Error;

use modify::Modify;

mod modify;

// Storage represents the internal-facing server part of TinyKV, it handles sending and receiving from other
// TinyKV nodes. As part of that responsibility, it also reads and writes data to disk (or semi-permanent memory).
trait Storage {
    fn start(&self) -> Result<(), Box<dyn Error>>;
    fn stop(&self) -> Result<(), Box<dyn Error>>;
    fn write(&self, batch: Vec<Modify>) -> Result<(), Box<dyn Error>>;
    fn reader(&self) -> Result<Box<dyn StorageReader>, Box<dyn Error>>;
}

trait StorageReader {
    fn get_cf(&self, cf: String, key: Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>>;
    fn iter_cf(&self, cf: String) -> dyn Iterator<Item=i32>;
    fn close(&self);
}
