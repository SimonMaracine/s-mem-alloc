use std::{ptr, collections::HashMap};

use crate::allocator::{Mem, Alloc};

pub struct BetterAllocator {
    memory: Mem,
    free_list: HashMap<usize, *mut u8>,
}

impl BetterAllocator {
    pub fn new(bytes: usize) -> Self {
        let memory = Mem::new(bytes);

        let mut list = HashMap::new();
        list.insert(bytes, memory.data);

        Self {
            memory,
            free_list: list,
        }
    }
}

impl Alloc for BetterAllocator {
    fn malloc(&mut self, bytes: usize) -> Option<*mut u8> {
        if bytes == 0 {
            return None;
        }



        Some(ptr::null_mut())
    }

    fn free(&mut self, address: *mut u8) {
        if address == ptr::null_mut() {
            return;
        }



    }
}
