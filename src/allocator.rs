use std::alloc::{alloc, dealloc, Layout};

pub (crate) trait Alloc {
    fn malloc(&mut self, bytes: usize) -> Option<*mut u8>;
    fn free(&mut self, address: *mut u8);
}

pub struct Mem {
    pub (crate) data: *mut u8,
    size: usize,
    layout: Layout,
}

impl Mem {
    pub fn new(size: usize) -> Self {
        let layout = Layout::new::<u8>();
        let data: *mut u8;

        unsafe {
            data = alloc(layout);
        }

        Self {
            data,
            size,
            layout,
        }
    }
}

impl Drop for Mem {
    fn drop(&mut self) {
        unsafe {
            dealloc(self.data, self.layout);
        }
    }
}
