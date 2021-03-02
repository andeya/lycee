use std::fs::File;
use std::io::Result;
use std::path::Path;

use crate::kv::storage::allocator::{allocator_s, ckid_t, lpid_t};
use crate::kv::storage::cache::cache_s;
use crate::kv::storage::inner::{busy_page_num_s, cursor_s, file_header_s, GPID_NIL, gpid_t, kvdb_assert, PAGE_BITMAP_PAGES, PAGE_NUM_PER_CK, PAGE_SIZE};
use crate::kv::storage::mmap::{CFile, MapT};

const FILE_HEADER_LEN: u64 = PAGE_SIZE as u64;

const OK: i8 = 0;
const PAGE_DELETED: i8 = 1;
const REC_NOT_FOUND: i8 = 2;
const PAGE_SPLITTED: i8 = 3;
const REC_REPLACED: i8 = 4;
const REC_INSERTED: i8 = 5;
const FOUND_EXACT: i8 = 6;
const FOUND_GREATER: i8 = 7;

pub struct kvdb_s {
    pub(crate) h: MapT<file_header_s>,
    pub alc: Option<allocator_s>,
    ch: cache_s,
    pub file: CFile,
}

impl kvdb_s {
    pub fn open<P: AsRef<Path>>(name: P) -> Result<kvdb_s> {
        let file = CFile::open(name)?;
        let new = file.metadata()?.len() < FILE_HEADER_LEN;
        let mut h = file.map_mut::<file_header_s>(0)?;
        let hd: &mut file_header_s = &mut *h;
        /* if the database is created right before, we should initialize the header of the file */
        if new {
            hd.magic = "kv@enmo";
            hd.record_num = 0;
            hd.root_gpid = GPID_NIL;
            hd.level = 0;
            hd.total_pages = 0;
            hd.spare_pages = 0;
        }
        hd.file_size = file.metadata()?.len();
        let mut db = kvdb_s {
            file,
            h,
            alc: None,
            ch: cache_s::new(),
        };
        db.init_allocator();
        return Ok(db);
    }
    pub fn get(&mut self, k: u64) -> Result<u64> {
        return unimplemented!();
        self.dump_page()?;
        // struct record_s rec;
        //
        // kvdb_dump_page(d, d->h->root_gpid);
        // ret = bpt_search(d, d->h->root_gpid, k, &rec, NULL);
        // if (ret==FOUND_EXACT) {
        // *v = rec.v;
        // return 0;
        // }
        // return -1;
    }
    pub fn put(&mut self, k: u64, v: u64) -> Result<()> {
        return unimplemented!();
        // if self.h.level == 0 {}
        // int kvdb_put(kvdb_t d, uint64_t k, uint64_t v)
        // {
        // 	int tries = 0;
        // 	struct record_s rec;
        // 	int ret;
        //
        // 	if (d->h->level==0) {
        // 		bpt_make_root(d, 1);
        // 	}
        // 	rec.k = k;
        // 	rec.v = v;
        // 	do {
        // 		ret = bpt_insert(d, NULL, NULL, -1, d->h->root_gpid, &rec);
        // 		tries ++;
        // 		kvdb_assert(tries<=2);
        // 	} while (ret==PAGE_SPLITED);
        //
        // 	if (ret!=REC_REPLACED) {
        // 		d->h->record_num ++;
        // 	}
        //
        // 	return 0;
        // }
    }
    pub fn del(&mut self, k: u64) -> Result<()> {
        Ok(())
    }
    pub fn iter(&self, start_key: u64, end_key: u64) -> Result<Box<cursor_s>> {
        unimplemented!()
    }
    pub fn dump(&self) -> Result<()> {
        println!("kvdb header:");
        return unimplemented!();
    }
    pub fn dump_page(&self) -> Result<()> {
        println!("kvdb header:");
        return unimplemented!();
        // pg_t pg;
        // 	struct page_s *p;
        //
        // 	pg = get_page(d, gpid);
        // 	p = get_page_buf(d, pg);
        // 	_kvdb_dump_page(gpid, p);
        // 	put_page(d, pg);
    }
    fn make_root(&mut self, leaf: i32) -> Result<()> {
        // struct page_s *p;
        // pg_t pg;
        // gpid_t gpid;
        //
        let gpid = self.alloc_page()?;
        kvdb_assert(gpid != GPID_NIL);
        self.h.root_gpid = gpid;
        self.h.level += 1;
        let pg = self.get_page(gpid);
        let p = self.get_page_buf(pg);
        Ok(())
        // p->h.record_num = 0;
        // p->h.flags = (leaf ? PAGE_LEAF : 0);
        // p->h.next = GPID_NIL;
        // put_page(d, pg);
    }
    fn alloc_page(&mut self) -> Result<gpid_t> {
        let alc = self.alc.as_mut().unwrap();
        let mut ck = alc.curr_ck;
        let mut gpid: gpid_t = 0;
        kvdb_assert(ck != ckid_t::MAX);
        /*
         * If there is not any free page in the chunk, then we find the next one
         * and turn to it
         */
        if alc.bpn.n[ck] >= PAGE_NUM_PER_CK {
            self.close_curr_ck()?;
            ck = self.find_ck(ck);
            /* TODO: reach the maximum length of the file, need to deal with it */
            kvdb_assert(ck != ckid_t::MAX);
            self.open_ck(ck);
        }
        let mut lpid_iter = PAGE_BITMAP_PAGES..PAGE_NUM_PER_CK;
        /* Find a free page in the chunk */
        for lpid in lpid_iter {
            gpid = kvdb_s::get_gpid(ck, lpid);
            if !self.pb_isset(gpid) {
                break;
            }
        }
        kvdb_assert(lpid_iter.next() != None);
        self.pb_set(gpid as lpid_t);
        alc.bpn.n[ck] += 1;
        // let pos = kvdb_s::get_page_pos(gpid);
        // if self.h.file_size < (pos + PAGE_SIZE) as u64 {
        //     self.file_allocate(pos, PAGE_SIZE);
        // }
        return Ok(gpid);
    }
    fn get_page(&self, p0: usize) -> usize {
        unimplemented!()
    }
    fn get_page_buf(&self, p0: usize) -> usize {
        unimplemented!()
    }
}


impl Iterator for cursor_s {
    type Item = (u64, u64);

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}

