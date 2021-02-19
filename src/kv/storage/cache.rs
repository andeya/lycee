use crate::kv::storage::inner::{page_s, PAGE_SIZE, gpid_t};

// 1MB for test
const MAX_CACHE_SIZE: u64 = 1 << 20;
const MAX_MAPPED_PG: u64 = MAX_CACHE_SIZE / PAGE_SIZE;
const EVECT_NUM: i32 = 128;

const PAGE_HASH_NUM: u64 = MAX_MAPPED_PG;
const PAGE_HASH_MASK: u64 = MAX_MAPPED_PG - 1;

const PG_DIRTY: u64 = 1 << 0;
const PG_BUSY: u64 = 1 << 1;

struct node_s {
    prev: Box<node_s>,
    next: Box<node_s>,
}

struct pg_s {
    // off:0
    flags: u32,
    reserv: u32,
    // off:8
    gpid: gpid_t,
    // off:16
    buf: Box<page_s>,
    // for hash, off:24
    hash: node_s,
    // for lru, off:40
    link: node_s,
}

pub struct cache_s {
    mapped_num: u64,
    busy_num: u64,
    free_num: u64,
    hash: [node_s; PAGE_HASH_NUM as usize],
    // free list head
    free: node_s,
    // busy list head
    busy: node_s,
}
