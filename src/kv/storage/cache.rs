use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::collections::LinkedList;
use std::io::Result;
use std::mem;
use std::ptr;
use std::rc::Rc;

use crate::kv::storage::inner::{gpid_t, Node, page_s, PAGE_SIZE};
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
    pub hash: Box<[Node; PAGE_HASH_NUM as usize]>,
    // free list head
    free: Option<Node>,
    // busy list head
    busy: Option<Node>,
}


impl cache_s {
    pub fn new() -> cache_s {
        const node: Node = Node::new();
        let c = cache_s {
            mapped_num: 0,
            busy_num: 0,
            free_num: 0,
            hash: Box::new([node; PAGE_HASH_NUM]),
            free: None,
            busy: None,
        };
        c
    }
    // fn list_add(h: &mut RcNode, mut n: ListNode) {
    //     let third = (&**h).borrow().next.clone();
    //     n.next = third.clone();
    //     n.prev = Some(Rc::clone(h));
    //     let mut n = Rc::new(RefCell::new(n));
    //     (&**h).borrow_mut().next = Some(Rc::clone(&n));
    //     if let Some(ref node) = third {
    //         (&**node).borrow_mut().prev = Some(n);
    //     }

    // golang:
    // third := h.next;
    // n.next = third;
    // n.prev = h;
    // h.next = n;
    // if third.prev != nil {
    //  third.prev = n;
    // }
    // }
}

