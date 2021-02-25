use std::mem;

use crate::{catch_backtrace, catch_symbol};

pub const PAGE_SIZE: usize = 4096;
pub const FILE_META_LEN: usize = 2 * 1024 * 1024;
pub const BUSY_PAGE_NUM_POS: usize = 1 * 1024 * 1024;
//bytes
pub const PAGE_BITMAP_LEN: usize = 64 * 1024;
pub const PAGE_BITMAP_PAGES: usize = PAGE_BITMAP_LEN / PAGE_SIZE;
pub const PAGE_NUM_PER_CK: usize = PAGE_BITMAP_LEN * 8;
//bytes
const PAGE_BITMAP_WLEN: usize = 64 * 1024 / 8;
pub const MAX_CHUNK_NUM: usize = 256 * 1024;
const CHUNK_DATA_LEN: usize = PAGE_BITMAP_LEN * 8 * PAGE_SIZE;
const DATA_AREA_LEN: usize = MAX_CHUNK_NUM * CHUNK_DATA_LEN;

const RECORD_NUM_PG: usize = (PAGE_SIZE / mem::size_of::<record_s>()) - 1;


// TODO: to dump all items in the call stack
pub fn kvdb_assert(cond: bool) {
    if !cond {
        println!("{:?}", catch_backtrace(1, 10));
    }
}

fn _pl() {
    eprintln!("{}", catch_symbol(1));
}

// global page id
pub type gpid_t = usize;

pub const GPID_NIL: gpid_t = gpid_t::MAX;

struct record_s {
    k: usize,
    v: usize,
}

const PAGE_LEAF: usize = 1 << 0;

struct page_header_s {
    record_num: i32,
    flags: u32,
    next: gpid_t,
}

pub struct page_s {
    h: page_header_s,
    rec: [record_s; RECORD_NUM_PG],
}

pub struct file_header_s {
    pub(crate) magic: &'static str,
    pub(crate) file_size: u64,
    pub(crate) record_num: usize,
    pub(crate) total_pages: usize,
    pub(crate) spare_pages: usize,
    pub(crate) level: u32,
    reserve: u32,
    pub(crate) root_gpid: gpid_t,
}

pub struct page_bitmap_s {
    pub(crate) w: [usize; PAGE_BITMAP_WLEN],
}


pub struct busy_page_num_s {
    pub(crate) n: [usize; MAX_CHUNK_NUM],
}


pub struct pg_s {
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

pub struct node_s {
    prev: Box<node_s>,
    next: Box<node_s>,
}


pub struct cursor_s {
    gpid: gpid_t,
    pg: Box<pg_s>,
    p: Box<page_s>,
    pos: i64,
    start_key: usize,
    end_key: usize,
}
