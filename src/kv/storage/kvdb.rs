use std::fs::File;
use std::io::Result;
use std::path::Path;

use crate::kv::storage::allocator::allocator_s;
use crate::kv::storage::cache::cache_s;
use crate::kv::storage::inner::{busy_page_num_s, cursor_s, file_header_s, GPID_NIL, PAGE_SIZE};
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
        Ok(0)
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
        Ok(())
    }
    pub fn del(&mut self, k: u64) -> Result<()> {
        Ok(())
    }
    pub fn iter(&self, start_key: u64, end_key: u64) -> Result<Box<cursor_s>> {
        unimplemented!()
    }
    pub fn dump(&self) -> Result<()> {
        println!("kvdb header:");
        unimplemented!()
    }
}


impl Iterator for cursor_s {
    type Item = (u64, u64);

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}

