use std::cell::RefCell;
use std::mem;
use std::ptr;
use std::ptr::NonNull;

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

#[derive(Debug, PartialEq)]
struct record_s {
    k: usize,
    v: usize,
}

const PAGE_LEAF: usize = 1 << 0;

#[derive(Debug, PartialEq)]
struct page_header_s {
    record_num: i32,
    flags: u32,
    next: gpid_t,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub struct pg_s {
    flags: u32,
    reserv: u32,
    gpid: gpid_t,
    buf: Option<page_s>,
    hash: Node,
    link: Node,
}

#[derive(Debug, PartialEq)]
pub struct Node {
    next: Option<NonNull<Node>>,
    prev: Option<NonNull<Node>>,
}

impl Node {
    pub(crate) const fn new() -> Node {
        Node { next: None, prev: None }
    }
}

const _PG: &pg_s = &pg_s::new();

impl pg_s {
    const fn new() -> pg_s {
        const r: record_s = record_s { k: 0, v: 0 };
        pg_s {
            flags: 0,
            reserv: 0,
            gpid: 0,
            buf: None,
            hash: Node::new(),
            link: Node::new(),
        }
    }
    pub fn from_hash(hash: &Node) -> &mut pg_s {
        unsafe {
            let PG_HASH_OFFSET = (&_PG.hash as *const Node as usize) - (_PG as *const pg_s as usize);// 16
            println!("PG_HASH_OFFSET={}", PG_HASH_OFFSET);
            &mut *((hash as *const Node as usize - PG_HASH_OFFSET) as *mut pg_s)
        }
    }
    pub fn from_link(link: &Node) -> &mut pg_s {
        unsafe {
            let PG_LINK_OFFSET = (&_PG.link as *const Node as usize) - (_PG as *const pg_s as usize);// 32
            println!("PG_LINK_OFFSET={}", PG_LINK_OFFSET);
            &mut *((link as *const Node as usize - PG_LINK_OFFSET) as *mut pg_s)
        }
    }
}

pub struct cursor_s {
    gpid: gpid_t,
    pg: Box<pg_s>,
    p: Box<page_s>,
    pos: i64,
    start_key: usize,
    end_key: usize,
}

#[cfg(test)]
mod tests {
    use crate::kv::storage::inner::pg_s;

    #[test]
    fn to_pg() {
        let pg = &mut pg_s::new();
        pg.flags = 101;
        let pg2 = pg_s::from_hash(&pg.hash);
        assert_eq!(101, pg2.flags);
        assert_eq!(pg, pg2);
        pg.flags = 202;
        let pg3 = pg_s::from_link(&pg.link);
        assert_eq!(202, pg3.flags);
        assert_eq!(pg, pg3);
    }
}
