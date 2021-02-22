use std::fs::File;

use crate::kv::storage::allocator::allocator_s;
use crate::kv::storage::cache::cache_s;
use crate::kv::storage::inner::{busy_page_num_s, file_header_s};
use crate::kv::storage::mmap::CFile;

pub struct kvdb_s {
    pub file: CFile,
    pub(crate) h: Box<file_header_s>,
    pub alc: Box<allocator_s>,
    ch: Box<cache_s>,
}
