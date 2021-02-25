use std::io::Result;

use crate::kv::storage::inner::{gpid_t, node_s, page_s, PAGE_SIZE};
use crate::kv::storage::kvdb::kvdb_s;

// 1MB for test
const MAX_CACHE_SIZE: usize = 1 << 20;
const MAX_MAPPED_PG: usize = MAX_CACHE_SIZE / PAGE_SIZE;
const EVECT_NUM: i32 = 128;

const PAGE_HASH_NUM: usize = MAX_MAPPED_PG;
const PAGE_HASH_MASK: usize = MAX_MAPPED_PG - 1;

const PG_DIRTY: usize = 1 << 0;
const PG_BUSY: usize = 1 << 1;


pub struct cache_s {
    mapped_num: usize,
    busy_num: usize,
    free_num: usize,
    hash: [node_s; PAGE_HASH_NUM as usize],
    // free list head
    free: node_s,
    // busy list head
    busy: node_s,
}

impl kvdb_s {
    pub fn init_cache(&mut self) -> Result<()> {
        Ok(())
    }
}
