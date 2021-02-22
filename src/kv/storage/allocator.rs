use std::{io, mem};

use crate::kv::storage::inner::{busy_page_num_s, page_bitmap_s};
use crate::kv::storage::kvdb::kvdb_s;
use crate::kv::storage::mmap::{CFile, MapT};

type ckid_t = u32;
//local page id
type lpid_t = u32;

const NULL_CK: u64 = u64::MAX;

pub struct allocator_s {
    curr_ck: ckid_t,
    bpn: MapT<busy_page_num_s>,
    pb: Box<page_bitmap_s>,
}

// impl kvdb_s {
//     fn init_allocator(&mut self, db: &mut kvdb_s) {
//         let mut new = false;
//         let ck: ckid_t = 0;
//         /*
// * if the file size smaller than the area of busy page number, the file is
// * a new one, so it is needed to be expanded.
// */
//         if db.h.file_size < BUSY_PAGE_NUM_POS + mem::size_of::<busy_page_num_s>() {
// // file_allocate(db, BUSY_PAGE_NUM_POS, sizeof(struct busy_page_num_s));
//             new = true;
//         }
//         self.alc = Box::new(allocator_s {
//             curr_ck: u32::MAX,
//             bpn: db.file.map_mut(BUSY_PAGE_NUM_POS)?,
//             pb: Box::new(page_bitmap_s::default()),
//         });
//
//         kvdb_assert(self.alc.bpn != MAP_FAILED);
//         if new {
//             memset(self.alc.bpn, 0, mem::size_of::<busy_page_num_s>());
//         }
//
//         ck = find_ck(db, 0);
//         kvdb_assert(ck != (ckid_t) - 1);
//
//         open_ck(db, ck);
//     }
// }


//
// void init_allocator(kvdb_t db)
// {
// struct allocator_s *alc;
// int new = 0;
// ckid_t ck;
//
// alc = (struct allocator_s *)malloc(sizeof(*alc));
// kvdb_assert(alc!=NULL);
// memset(alc, 0, sizeof(*alc));
// db->alc = alc;
// alc->curr_ck = (ckid_t)-1;
//
// /*
//  * if the file size smaller than the area of busy page number, the file is
//  * a new one, so it is needed to be expanded.
//  */
// if (db->h->file_size < BUSY_PAGE_NUM_POS + sizeof(struct busy_page_num_s)) {
// file_allocate(db, BUSY_PAGE_NUM_POS, sizeof(struct busy_page_num_s));
// new = 1;
// }
//
// alc->bpn = mmap(NULL, sizeof(struct busy_page_num_s),
// PROT_READ|PROT_WRITE, MAP_SHARED,
// db->fd, BUSY_PAGE_NUM_POS);
// kvdb_assert(alc->bpn!=MAP_FAILED);
//
// if (new)
// memset(alc->bpn, 0, sizeof(struct busy_page_num_s));
//
// ck = find_ck(db, 0);
// kvdb_assert(ck!=(ckid_t)-1);
//
// open_ck(db, ck);
// }
