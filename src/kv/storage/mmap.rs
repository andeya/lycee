use std::{io, ptr};
use std::fmt::Debug;
use std::fs::{File, OpenOptions};
use std::path::Path;

use mmapio::{AsMutT, AsRefT, Mmap, MmapOptions};

pub struct CFile(File);

#[derive(Debug)]
pub struct RefT<'a, T> {
    t: &'a T,
    owner: Mmap,
}

impl<'a, T> AsRef<T> for RefT<'a, T> {
    fn as_ref(&self) -> &'a T {
        self.t
    }
}

impl CFile {
    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<CFile> {
        Ok(CFile(open_file(path)?))
    }

    pub fn c_write_raw<T>(&self, offset: u64, src: &T) -> io::Result<()> {
        unsafe {
            let mut mmap = MmapOptions::new()
                .len(std::mem::size_of::<T>())
                .offset(offset)
                .map_mut(&self.0)?;
            mmap.write_t(src);
        }
        Ok(())
    }

    pub fn c_read_raw<'a, T: Debug>(&self, offset: u64) -> io::Result<RefT<'a, T>> {
        unsafe {
            let mmap = MmapOptions::new()
                .len(std::mem::size_of::<T>())
                .offset(offset)
                .map(&self.0)?;
            Ok(RefT { t: mmap.as_ref_t::<'a, T>(), owner: mmap })
        }
    }
}


fn open_file<P: AsRef<Path>>(path: P) -> io::Result<File> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
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
    use std::sync::Once;

    use crate::kv::storage::mmap::{CFile, RefT};

    #[repr(C)]
    #[derive(Debug)]
    struct A {
        n: [u8; 128],
    }

    #[test]
    fn test_write() {
        let mut f = CFile::open("test.mmap")
            .expect("Unable to open file");
        let mut src = A { n: [0; 128] };
        src.n[0..4].copy_from_slice(&[2, 3, 4, 8]);
        f.c_write_raw(0, &src).expect("c_write_raw");
        f.0.flush();
    }

    #[test]
    fn test_read() {
        let f = CFile::open("test.mmap")
            .expect("Unable to open file");
        let src: RefT<A> = f.c_read_raw(0).expect("c_read_raw");
        println!("size={}\nsrc={:?}", mem::size_of::<A>(), src.as_ref());
    }
}
