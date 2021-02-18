use std::backtrace::Backtrace;
use std::mem;

use crate::format_backtrace;

const PAGE_SIZE: u64 = 4096;
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

const RECORD_NUM_PG: u64 = (PAGE_SIZE / mem::size_of::<record_s>()) - 1;

struct record_s {
    k: uint64_t,
    v: uint64_t,
}

// TODO: to dump all items in the call stack
fn kvdb_assert(cond: bool) {
    if !cond {
        println!("{}", format_backtrace(Some(1), Some(10)));
    }
}

fn _pl() {
    eprintln!("func={}(), at %s:{}", Backtrace::capture());
}
