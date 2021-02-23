use std::{io, ptr};
use std::default::Default;
use std::fmt::Debug;
use std::fs::{File, Metadata, OpenOptions};
use std::io::{Result, Seek};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::path::Path;

use mmapio::{AsMutT, AsRefT, MmapMut, MmapOptions};

pub struct CFile(File);


impl CFile {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<CFile> {
        Ok(CFile(OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?))
    }
    pub fn metadata(&self) -> Result<Metadata> {
        self.0.metadata()
    }
    pub fn map_mut<T>(&self, offset: u64) -> Result<MapT<T>> {
        unsafe {
            MapT::<T>::new(&self.0, offset)
        }
    }
}


fn copy_nonoverlapping<S, D>(src: *const S, dst: *mut D) {
    unsafe { ptr::copy_nonoverlapping(src as *const u8, dst as *mut u8, std::mem::size_of::<S>()); }
}

fn transmute_ref<'a, P, T>(p: *const P) -> &'a T {
    unsafe { &*(p as *const T) }
}

#[derive(Debug)]
pub struct MapT<T>(MmapMut, PhantomData<T>);

impl<T> Default for MapT<T> {
    fn default() -> MapT<T> {
        MapT(MmapMut::default(), PhantomData)
    }
}

impl<T> MapT<T> {
    pub fn set(&mut self, src: &T) {
        unsafe { self.0.write_t(src) }
    }
    pub unsafe fn new(file: &File, offset: u64) -> Result<MapT<T>> {
        Ok(MapT(MmapOptions::new()
                    .len(std::mem::size_of::<T>())
                    .offset(offset)
                    .map_mut(file)?,
                PhantomData::<T>,
        ))
    }
    /// Flushes outstanding memory map modifications to disk.
    ///
    /// When this method returns with a non-error result, all outstanding changes to a file-backed
    /// memory map are guaranteed to be durably stored. The file's metadata (including last
    /// modification timestamp) may not be updated.
    ///
    /// # Example
    ///
    /// ```
    /// use std::fs::OpenOptions;
    /// use std::io::Write;
    /// use std::path::PathBuf;
    ///
    /// use mmapio::MmapMut;
    ///
    /// # fn main() -> std::io::Result<()> {
    /// # let tempdir = tempdir::TempDir::new("mmap")?;
    /// let path: PathBuf = /* path to file */
    /// #   tempdir.path().join("flush");
    /// let file = OpenOptions::new().read(true).write(true).create(true).open(&path)?;
    /// file.set_len(128)?;
    ///
    /// let mut mmap = unsafe { MmapMut::map_mut(&file)? };
    ///
    /// (&mut mmap[..]).write_all(b"Hello, world!")?;
    /// mmap.flush()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn flush(&self) -> Result<()> {
        self.0.flush()
    }

    /// Asynchronously flushes outstanding memory map modifications to disk.
    ///
    /// This method initiates flushing modified pages to durable storage, but it will not wait for
    /// the operation to complete before returning. The file's metadata (including last
    /// modification timestamp) may not be updated.
    pub fn flush_async(&self) -> Result<()> {
        self.0.flush_async()
    }

    /// Flushes outstanding memory map modifications in the range to disk.
    ///
    /// The offset and length must be in the bounds of the memory map.
    ///
    /// When this method returns with a non-error result, all outstanding changes to a file-backed
    /// memory in the range are guaranteed to be durable stored. The file's metadata (including
    /// last modification timestamp) may not be updated. It is not guaranteed the only the changes
    /// in the specified range are flushed; other outstanding changes to the memory map may be
    /// flushed as well.
    pub fn flush_range(&self, offset: usize, len: usize) -> Result<()> {
        self.flush_range(offset, len)
    }

    /// Asynchronously flushes outstanding memory map modifications in the range to disk.
    ///
    /// The offset and length must be in the bounds of the memory map.
    ///
    /// This method initiates flushing modified pages to durable storage, but it will not wait for
    /// the operation to complete before returning. The file's metadata (including last
    /// modification timestamp) may not be updated. It is not guaranteed that the only changes
    /// flushed are those in the specified range; other outstanding changes to the memory map may
    /// be flushed as well.
    pub fn flush_async_range(&self, offset: usize, len: usize) -> Result<()> {
        self.flush_async_range(offset, len)
    }
}

impl<T> Deref for MapT<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref_t() }
    }
}

impl<T> DerefMut for MapT<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut_t() }
    }
}

#[cfg(test)]
mod tests {
    use std::fs::OpenOptions;
    use std::io::Write;
    use std::mem;
    use std::sync::Once;

    use mmapio::AsMutT;

    use crate::kv::storage::mmap::{CFile, MapT};

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
        let mut mmap: MapT<A> = f.map_mut(0)
                                 .expect("write");
        mmap.set(&src);
        mmap.flush();
    }

    #[test]
    fn test_read() {
        let f = CFile::open("test.mmap")
            .expect("Unable to open file");
        let mut a: &mut A = &mut f.map_mut(0)
                                  .expect("read");
        a.n[0] = 12;
        println!("size={}\nsrc={:?}", mem::size_of::<A>(), a);
    }
}
