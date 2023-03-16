use std::{ptr, collections::LinkedList};

use crate::allocator::{Mem, Alloc};

pub struct NaiveAllocator {
    memory: Mem,
    free_list: LinkedList<FreeBlock>,
}

impl NaiveAllocator {
    pub fn new(bytes: usize) -> Self {
        let memory = Mem::new(bytes);

        let mut list = LinkedList::new();
        list.push_back(FreeBlock { address: memory.data, size: bytes });

        Self {
            memory,
            free_list: list,
        }
    }

    fn allocate_block_full(&mut self) {

    }

    fn allocate_block_partial(&mut self) {
        
    }
}

impl Alloc for NaiveAllocator {
    fn malloc(&mut self, bytes: usize) -> Option<*mut i8> {
        if bytes == 0 {
            return None;
        }

        for list in &self.free_list {
            if list.size > bytes {
                self.allocate_block_partial();
            } else if list.size == bytes {
                self.allocate_block_full();
            }
        }

        Some(ptr::null_mut())
    }

    fn free(&mut self, address: *mut u8) {
        if address == ptr::null_mut() {
            return;
        }



    }
}

struct FreeBlock {
    address: *mut u8,
    size: usize,
}
