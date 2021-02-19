use std::mem;

use crate::{catch_backtrace, catch_symbol};

pub const PAGE_SIZE: u64 = 4096;
const FILE_META_LEN: u64 = 2 * 1024 * 1024;
const BUSY_PAGE_NUM_POS: u64 = 1 * 1024 * 1024;
//bytes
const PAGE_BITMAP_LEN: u64 = 64 * 1024;
const PAGE_BITMAP_PAGES: u64 = PAGE_BITMAP_LEN / PAGE_SIZE;
const PAGE_NUM_PER_CK: u64 = PAGE_BITMAP_LEN * 8;
//bytes
const PAGE_BITMAP_WLEN: u64 = 64 * 1024 / 8;
const MAX_CHUNK_NUM: u64 = 256 * 1024;
const CHUNK_DATA_LEN: u64 = PAGE_BITMAP_LEN * 8 * PAGE_SIZE;
const DATA_AREA_LEN: u64 = MAX_CHUNK_NUM * CHUNK_DATA_LEN;

const RECORD_NUM_PG: u64 = (PAGE_SIZE / mem::size_of::<record_s>() as u64) - 1;


// TODO: to dump all items in the call stack
fn kvdb_assert(cond: bool) {
    if !cond {
        println!("{:?}", catch_backtrace(1, 10));
    }
}

fn _pl() {
    eprintln!("{}", catch_symbol(1));
}

// global page id
pub type gpid_t = u64;

const GPID_NIL: gpid_t = gpid_t::MAX;

struct record_s {
    k: u64,
    v: u64,
}

const PAGE_LEAF: u64 = 1 << 0;

struct page_header_s {
    record_num: i32,
    flags: u32,
    next: gpid_t,
}

pub struct page_s {
    h: page_header_s,
    rec: [record_s; RECORD_NUM_PG as usize],
}

pub struct file_header_s {
    magic: u64,
    file_size: u64,
    record_num: u64,
    total_pages: u64,
    spare_pages: u64,
    level: u32,
    reserve: u32,
    root_gpid: gpid_t,
}

pub struct page_bitmap_s {
    w: [u64; PAGE_BITMAP_WLEN as usize],
}

pub struct busy_page_num_s {
    n: [u32; MAX_CHUNK_NUM as usize],
}


struct pg_s;

type pg_t = Box<pg_s>;


// pub type kvdb_t = &mut kvdb_s;

struct cursor_s {
    gpid: gpid_t,
    pg: pg_t,
    p: Box<page_s>,
    pos: i64,
    start_key: u64,
    end_key: u64,
}
