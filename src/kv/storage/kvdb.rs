use crate::kv::storage::allocator::allocator_s;
use crate::kv::storage::cache::cache_s;
use crate::kv::storage::inner::file_header_s;

struct kvdb_s {
    fd: i64,
    h: Box<file_header_s>,
    alc: Box<allocator_s>,
    ch: Box<cache_s>,
}
