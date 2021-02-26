use std::{io, mem};
use std::io::Result;

use crate::kv::storage::inner::{BUSY_PAGE_NUM_POS, busy_page_num_s, FILE_META_LEN, gpid_t, kvdb_assert, MAX_CHUNK_NUM, PAGE_BITMAP_LEN, PAGE_BITMAP_PAGES, page_bitmap_s, PAGE_NUM_PER_CK, PAGE_SIZE};
use crate::kv::storage::kvdb::kvdb_s;
use crate::kv::storage::mmap::{CFile, MapT};

type ckid_t = usize;
//local page id
type lpid_t = usize;


pub struct allocator_s {
    curr_ck: ckid_t,
    bpn: MapT<busy_page_num_s>,
    pb: MapT<page_bitmap_s>,
}

impl kvdb_s {
    pub fn init_allocator(&mut self) -> Result<()> {
        let mut ck: ckid_t = 0;
        /*
        * if the file size smaller than the area of busy page number, the file is
        * a new one, so it is needed to be expanded.
        */
        self.alc = Some(allocator_s {
            curr_ck: ckid_t::MAX,
            bpn: self.file.map_mut(BUSY_PAGE_NUM_POS as u64)?,
            pb: MapT::<page_bitmap_s>::default(),
        });

        if self.h.file_size < BUSY_PAGE_NUM_POS as u64 + mem::size_of::<busy_page_num_s>() as u64 {
            self.h.file_size = self.file.metadata()?.len()
        }

        ck = self.find_ck(0);
        kvdb_assert(ck != ckid_t::MAX);

        self.open_ck(ck)
    }

    /*
     * open_ck() -- load a page bitmap into memory. At any moment, there is only
     * 				one ck could be staying in the memory to provide free pages.
     */
    fn open_ck(&mut self, ck: ckid_t) -> Result<()> {
        let mut pos: usize;
        if let Some(ref mut alc) = self.alc {
            kvdb_assert(alc.curr_ck == ckid_t::MAX);
            kvdb_assert(ck != ckid_t::MAX);

            alc.curr_ck = ck;
            pos = Self::get_ck_pos(ck);

            alc.pb = self.file.map_mut(pos as u64)?;

            if alc.bpn.n[ck as usize] == 0 {
                if self.h.file_size < pos as u64 + PAGE_BITMAP_LEN as u64 {
                    self.h.file_size = self.file.metadata()?.len()
                }
                alc.bpn.n[ck as usize] = PAGE_BITMAP_PAGES;
                for i in 0..PAGE_BITMAP_PAGES {
                    self.pb_set(i as lpid_t);
                }
            }
        }
        Ok(())
    }


    fn pb_set(&mut self, pg: lpid_t) {
        let w = pg >> 6;
        let b = pg & 63;
        if let Some(ref mut alc) = self.alc {
            alc.pb.w[w as usize] |= 1 << b;
        }
    }

    /* find a chunk which has free pages to allocate */
    fn find_ck(&mut self, ck: ckid_t) -> ckid_t {
        if let Some(ref alc) = self.alc {
            for i in 0..MAX_CHUNK_NUM {
                let r = (ck + i) % MAX_CHUNK_NUM;
                if alc.bpn.n[i] < PAGE_NUM_PER_CK {
                    return r;
                }
            }
        }
        return ckid_t::MAX;
    }


    fn get_gpid(ck: ckid_t, lpid: lpid_t) -> gpid_t {
        (ck as gpid_t) * PAGE_NUM_PER_CK + lpid as usize
    }

    fn get_page_pos(gpid: gpid_t) -> usize {
        FILE_META_LEN + gpid * PAGE_SIZE
    }

    fn get_ck_pos(ck: ckid_t) -> usize {
        let gpid = Self::get_gpid(ck, 0);
        let pos = Self::get_page_pos(gpid);
        return pos;
    }
}

