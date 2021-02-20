use std::{
    ffi::c_void,
    fs::File,
    io::{Seek, SeekFrom, Write},
    mem,
    os::unix::prelude::AsRawFd,
    ptr,
};

use libc;

pub fn c_read_raw<'a, T>(f: &mut File) -> &'a T {
    transmute_ref(c_rw_file::<T>(f))
}

pub fn c_write_raw<T>(src: &T, f: &mut File) {
    copy_nonoverlapping(src, c_rw_file::<T>(f));
}

fn c_rw_file<T>(f: &mut File) -> *mut c_void {
    let size = mem::size_of::<T>();
    // Allocate space in the file first
    f.seek(SeekFrom::Start(size as u64)).unwrap();
    f.write_all(&[0]).unwrap();
    f.seek(SeekFrom::Start(0)).unwrap();
    // This refers to the `File` but doesn't use lifetimes to indicate
    // that. This is very dangerous, and you need to be careful.
    unsafe {
        let data = libc::mmap(
            /* addr: */ ptr::null_mut(),
            /* len: */ size,
            /* prot: */ libc::PROT_READ | libc::PROT_WRITE,
            // Then make the mapping *public* so it is written back to the file
            /* flags: */ libc::MAP_SHARED,
            /* fd: */ f.as_raw_fd(),
            /* offset: */ 0,
        );
        if data == libc::MAP_FAILED {
            panic!("could not access data from memory mapped file")
        }
        data as *mut c_void
    }
}


pub fn copy_nonoverlapping<S, D>(src: *const S, dst: *mut D) {
    unsafe { ptr::copy_nonoverlapping(src as *const u8, dst as *mut u8, std::mem::size_of::<S>()); }
}


pub fn transmute_ref<'a, P, T>(p: *const P) -> &'a T {
    unsafe { &*(p as *const T) }
}

#[cfg(test)]
mod tests {
    use std::fs::OpenOptions;
    use std::io::Write;
    use std::mem;

    use crate::kv::storage::mmap::{c_read_raw, c_write_raw};

    #[repr(C)]
    #[derive(Debug)]
    struct A {
        n: [u8; 128],
    }

    #[test]
    fn test_write() {
        let mut f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("test.mmap")
            .expect("Unable to open file");
        let mut src = A { n: [0; 128] };
        src.n[0..4].copy_from_slice(&[2, 3, 4, 8]);
        c_write_raw(&src, &mut f);
        f.flush();
    }

    #[test]
    fn test_read() {
        let mut f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("test.mmap")
            .expect("Unable to open file");

        let src: &A = c_read_raw(&mut f);
        println!("size={}\nsrc={:?}", mem::size_of::<A>(), src.clone());
    }
}
